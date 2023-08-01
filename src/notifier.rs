use serenity::{
    model::{channel::Message, id::ChannelId},
    prelude::*,
};
use std::sync::Arc;

enum ChannelState {
    Uninitialized,
    Notified,
    NotNotified,
}

pub struct Notifier {
    ctx: Arc<Context>,
    channel_id: u64,
}

impl Notifier {
    pub async fn get_messages(&self) -> Vec<Message> {
        let channel_id = ChannelId(self.channel_id);
        channel_id
            .messages(&self.ctx.http, |retriever| retriever.limit(100))
            .await
            .expect("Failed to fetch messages")
    }

    pub async fn get_state() {}

    pub fn new(ctx: Arc<Context>, channel_id: u64) -> Notifier {
        Notifier { ctx, channel_id }
    }
}
