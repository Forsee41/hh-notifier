use crate::{crontime::CronTime, messages_generator::MessageManager, notifier::Notifier};
use serenity::{async_trait, model::gateway::Ready, prelude::*};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_cron_scheduler::{Job, JobScheduler};

pub struct BotConfig {
    pub role_id: u64,
    pub start: u64,
    pub end: u64,
    pub notify_shift: u64,
    pub channel_id: u64,
}

pub struct Handler {
    scheduler: JobScheduler,
    notifier: Arc<Mutex<Option<Notifier>>>,
    msg_manager: Arc<MessageManager>,
    config: Arc<BotConfig>,
    crontime: Arc<CronTime>,
}

impl Handler {
    pub fn new(scheduler: JobScheduler, config: BotConfig) -> Self {
        let crontime = CronTime::new(config.start, config.end, config.notify_shift).unwrap();
        let config_arc = Arc::new(config);
        let arc_clone = Arc::clone(&config_arc);
        let msg_manager = MessageManager::new(config_arc);

        Handler {
            scheduler,
            notifier: Arc::new(Mutex::new(None)),
            msg_manager: Arc::new(msg_manager),
            config: arc_clone,
            crontime: Arc::new(crontime),
        }
    }
    fn update_job(&self) -> Job {
        let update_crontime = &self.crontime.update()[..];
        let notifier_clone = Arc::clone(&self.notifier);
        Job::new_async(update_crontime, move |_uuid, _l| {
            let notifier_clone = Arc::clone(&notifier_clone);
            Box::pin(async move {
                notifier_clone
                    .lock()
                    .await
                    .as_mut()
                    .unwrap()
                    .update_time()
                    .await;
            })
        })
        .expect("Couldn't create a cronjob")
    }
    fn notify_job(&self) -> Job {
        let notify_crontime = &self.crontime.notify()[..];
        let notifier_clone = Arc::clone(&self.notifier);
        Job::new_async(notify_crontime, move |_uuid, _l| {
            let notifier_clone = Arc::clone(&notifier_clone);
            Box::pin(async move {
                notifier_clone.lock().await.as_mut().unwrap().notify().await;
            })
        })
        .expect("Couldn't create a cronjob")
    }
    fn end_job(&self) -> Job {
        let end_crontime = &self.crontime.end()[..];
        let notifier_clone = Arc::clone(&self.notifier);
        Job::new_async(end_crontime, move |_uuid, _l| {
            let notifier_clone = Arc::clone(&notifier_clone);
            Box::pin(async move {
                notifier_clone
                    .lock()
                    .await
                    .as_mut()
                    .unwrap()
                    .unnotify()
                    .await;
            })
        })
        .expect("Couldn't create a cronjob")
    }
    async fn reg_jobs(&self) {
        self.scheduler
            .add(self.update_job())
            .await
            .expect("Couldn't add an update job to the scheduler");
        self.scheduler
            .add(self.notify_job())
            .await
            .expect("Couldn't add a notify job to the scheduler");
        self.scheduler
            .add(self.end_job())
            .await
            .expect("Couldn't add an end job to the scheduler");
    }
    async fn start_scheduler(&self) {
        self.scheduler
            .start()
            .await
            .expect("Couldn't start a scheduler execution");
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("Bot is ready, user: {}", ready.user.name);
        let ctx_arc = Arc::new(ctx);
        let bot_uid = ready.user.id;
        let msg_manager = Arc::clone(&self.msg_manager);
        let config = Arc::clone(&self.config);
        let notifier = Notifier::new(ctx_arc, bot_uid, msg_manager, config).await;
        *self.notifier.lock().await = Some(notifier);
        self.reg_jobs().await;
        self.start_scheduler().await;
    }
}
