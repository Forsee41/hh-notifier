use chrono::{prelude::*, Duration};
use serenity::{
    async_trait,
    model::{channel::Message, id::ChannelId, prelude::UserId},
    prelude::*,
};
use std::sync::Arc;

#[async_trait]
trait State: Send + Sync {
    async fn update_time(&self, notifier: &Notifier) -> ();
    async fn notify(&self, notifier: &Notifier) -> ();
    async fn unnotify(&self, notifier: &Notifier) -> ();
}

struct Uninitialized;

#[async_trait]
impl State for Uninitialized {
    async fn update_time(&self, notifier: &Notifier) -> () {}
    async fn notify(&self, notifier: &Notifier) -> () {
        println!("Notifying users!");
    }
    async fn unnotify(&self, notifier: &Notifier) -> () {}
}

struct Notified;

#[async_trait]
impl State for Notified {
    async fn update_time(&self, notifier: &Notifier) -> () {}
    async fn notify(&self, notifier: &Notifier) -> () {}
    async fn unnotify(&self, notifier: &Notifier) -> () {}
}

struct NonNotified;

#[async_trait]
impl State for NonNotified {
    async fn update_time(&self, notifier: &Notifier) -> () {}
    async fn notify(&self, notifier: &Notifier) -> () {}
    async fn unnotify(&self, notifier: &Notifier) -> () {}
}

struct MessageManager {
    start: u64,
    end: u64,
    shift: u64,
    tz_shift: u64,
}
impl MessageManager {
    fn now_tz(&self) -> DateTime<FixedOffset> {
        let tz = self.tz();
        let utc_now = Utc::now();
        utc_now.with_timezone(&tz)

    }
    fn tz(&self) -> FixedOffset {
        let hour = 3600;
        let tz_shift_seconds: i32 = hour * self.tz_shift as i32;
        FixedOffset::east_opt(tz_shift_seconds).unwrap()
    }
    fn next_hh_start_datetime(&self) -> DateTime<FixedOffset> {
        let now = self.now_tz();
        let hh_start = now.clone().date_naive().and_hms_opt(self.start as u32, 0, 0).unwrap();
        let hh_start_with_tz: DateTime<FixedOffset> = DateTime::<FixedOffset>::from_utc(hh_start, self.tz()).with_timezone(&self.tz());
        if now.hour() as u64 >= self.start {
            hh_start_with_tz + Duration::days(1)
        } else {
            hh_start_with_tz
        }
    }
    fn duration_until_next_hh_start(&self) {}
    async fn generate_info_str(&self) -> String {
        "".to_owned()
    }
}

pub struct Notifier {
    ctx: Arc<Context>,
    channel_id: u64,
    bot_uid: UserId,
}

impl Notifier {
    pub async fn get_messages(&self) -> Vec<Message> {
        let channel_id = ChannelId(self.channel_id);
        channel_id
            .messages(&self.ctx.http, |retriever| retriever.limit(100))
            .await
            .expect("Failed to fetch messages")
    }

    async fn state(&self, messages: &Vec<Message>) -> Box<dyn State> {
        if (1..=2).contains(&messages.len()) {
            return Box::new(Uninitialized);
        };
        for msg in messages {
            if msg.author.id != self.bot_uid {
                return Box::new(Uninitialized);
            }
        }
        Box::new(NonNotified)
    }

    pub async fn update_time(&mut self) {
        let messages = self.get_messages().await;
        let state = self.state(&messages).await;
        state.update_time(self).await;
    }

    pub async fn notify(&mut self) {
        let messages = self.get_messages().await;
        let state = self.state(&messages).await;
        state.notify(self).await;
    }

    pub async fn unnotify(&mut self) {
        let messages = self.get_messages().await;
        let state = self.state(&messages).await;
        state.unnotify(self).await;
    }

    pub async fn new(ctx: Arc<Context>, channel_id: u64, bot_uid: UserId) -> Notifier {
        Notifier {
            ctx,
            channel_id,
            bot_uid,
        }
    }
}
