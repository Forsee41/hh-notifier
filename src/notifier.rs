use serenity::{
    async_trait,
    model::{channel::Message, id::ChannelId, prelude::UserId},
    prelude::*,
};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{handler::BotConfig, messages_generator::MessageManager};

#[async_trait]
trait State: Send + Sync {
    async fn update_time(&self, notifier: &Notifier, messages: Arc<Mutex<Vec<Message>>>) -> ();
    async fn notify(&self, notifier: &Notifier, messages: Arc<Mutex<Vec<Message>>>) -> ();
    async fn unnotify(&self, notifier: &Notifier, messages: Arc<Mutex<Vec<Message>>>) -> ();
}

struct Uninitialized;

#[async_trait]
impl State for Uninitialized {
    async fn update_time(&self, notifier: &Notifier, messages: Arc<Mutex<Vec<Message>>>) -> () {
        let http = notifier.ctx.http.clone();
        for msg in messages.lock().await.iter() {
            msg.delete(http.clone())
                .await
                .expect("Can't delete the message");
        }
        let channel = notifier.channel();
        let msg_str = notifier.msg_manager.info_str();
        channel
            .send_message(http, |m| m.content(msg_str))
            .await
            .expect("Can't send the message in update_time");
    }
    async fn notify(&self, notifier: &Notifier, messages: Arc<Mutex<Vec<Message>>>) -> () {
        let http = notifier.ctx.http.clone();
        let msg_str = notifier.msg_manager.notify_str();
        let role_id = notifier.config.role_id;
        self.update_time(notifier, messages).await;
        notifier
            .channel()
            .send_message(http, |m| {
                m.content(msg_str)
                    .allowed_mentions(|a| a.empty_parse().roles(vec![role_id]))
            })
            .await
            .expect("Can't send the message in uninitialized notify");
    }
    async fn unnotify(&self, notifier: &Notifier, messages: Arc<Mutex<Vec<Message>>>) -> () {
        self.update_time(notifier, messages).await
    }
}

struct Notified;

#[async_trait]
impl State for Notified {
    async fn update_time(&self, notifier: &Notifier, messages: Arc<Mutex<Vec<Message>>>) -> () {
        let http = notifier.ctx.http.clone();
        let msg_str = notifier.msg_manager.info_str();
        let msg = &mut messages.lock().await[1];
        msg.edit(&http, |m| m.content(msg_str)).await.expect("Couldn't edit message in update_time notified state");
    }
    async fn notify(&self, _: &Notifier, _: Arc<Mutex<Vec<Message>>>) -> () {}
    async fn unnotify(&self, notifier: &Notifier, messages: Arc<Mutex<Vec<Message>>>) -> () {
        let http = notifier.ctx.http.clone();
        let msg = &mut messages.lock().await[0];
        msg.delete(&http).await.expect("Couldn't delete notification message in notified unnotify");
    }
}

struct NonNotified;

#[async_trait]
impl State for NonNotified {
    async fn update_time(&self, notifier: &Notifier, messages: Arc<Mutex<Vec<Message>>>) -> () {
        let http = notifier.ctx.http.clone();
        let msg_str = notifier.msg_manager.info_str();
        let msg = &mut messages.lock().await[0];
        msg.edit(&http, |m| m.content(msg_str)).await.expect("Couldn't edit message in update_time notified state");

    }
    async fn notify(&self, notifier: &Notifier, _: Arc<Mutex<Vec<Message>>>) -> () {
        let http = notifier.ctx.http.clone();
        let msg_str = notifier.msg_manager.notify_str();
        let role_id = notifier.config.role_id;
        notifier
            .channel()
            .send_message(http, |m| {
                m.content(msg_str)
                    .allowed_mentions(|a| a.empty_parse().roles(vec![role_id]))
            })
            .await
            .expect("Can't send the message in uninitialized notify");
    }
    async fn unnotify(&self, _: &Notifier, _: Arc<Mutex<Vec<Message>>>) -> () {}
}

pub struct Notifier {
    msg_manager: Arc<MessageManager>,
    ctx: Arc<Context>,
    config: Arc<BotConfig>,
    bot_uid: UserId,
}

impl Notifier {
    pub async fn get_messages(&self) -> Vec<Message> {
        let channel_id = self.channel();
        channel_id
            .messages(&self.ctx.http, |retriever| retriever.limit(100))
            .await
            .expect("Failed to fetch messages")
    }

    async fn state(&self, messages: &Vec<Message>) -> Box<dyn State> {
        if !(1..=2).contains(&messages.len()) {
            return Box::new(Uninitialized);
        };
        for msg in messages {
            if msg.author.id != self.bot_uid {
                return Box::new(Uninitialized);
            }
        }
        if messages.len() == 1 {
            return Box::new(NonNotified)
        }
        Box::new(Notified)
    }
    pub fn channel(&self) -> ChannelId {
        ChannelId(self.config.channel_id)
    }

    pub async fn update_time(&mut self) {
        let messages = self.get_messages().await;
        let state = self.state(&messages).await;
        state.update_time(self, Arc::new(Mutex::new(messages))).await;
    }

    pub async fn notify(&mut self) {
        let messages = self.get_messages().await;
        let state = self.state(&messages).await;
        state.notify(self, Arc::new(Mutex::new(messages))).await;
    }

    pub async fn unnotify(&mut self) {
        let messages = self.get_messages().await;
        let state = self.state(&messages).await;
        state.unnotify(self, Arc::new(Mutex::new(messages))).await;
    }

    pub async fn new(
        ctx: Arc<Context>,
        bot_uid: UserId,
        msg_manager: Arc<MessageManager>,
        config: Arc<BotConfig>,
    ) -> Notifier {
        Notifier {
            ctx,
            bot_uid,
            msg_manager,
            config,
        }
    }
}
