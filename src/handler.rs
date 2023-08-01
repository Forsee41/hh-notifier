use serenity::{
    async_trait,
    model::gateway::Ready,
    prelude::*,
};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_cron_scheduler::{Job, JobScheduler};
use crate::notifier::Notifier;

pub struct Handler {
    channel_id: u64,
    scheduler: JobScheduler,
    notifier: Arc<Mutex<Option<Notifier>>>,
}

impl Handler {
    pub fn new(channel_id: u64, scheduler: JobScheduler) -> Self {
        Handler {
            channel_id,
            scheduler,
            notifier: Arc::new(Mutex::new(None)),
        }
    }
}


#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("Bot is ready, user: {}", ready.user.name);
        let ctx_arc = Arc::new(ctx);
        let notifier = Notifier::new(ctx_arc, self.channel_id);
        *self.notifier.lock().await = Some(notifier);
        let messages = self
            .notifier
            .lock()
            .await
            .as_ref()
            .unwrap()
            .get_messages()
            .await;
        println!("{}", messages[0].content);
        let job = Job::new_async("* * * * * *", |_uuid, _l| {
            Box::pin(async move {
                println!("Doing a job");
            })
        })
        .expect("Couldn't create a cronjob");
        self.scheduler
            .add(job)
            .await
            .expect("Couldn't add a job to the scheduler");
        self.scheduler
            .start()
            .await
            .expect("Couldn't start a scheduler execution");
    }
}
