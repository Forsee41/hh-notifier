use dotenv::dotenv;
use hh_notifier::handler::Handler;
use serenity::prelude::{Client, GatewayIntents};
use tokio_cron_scheduler::JobScheduler;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = std::env::var("DISCORD_BOT_TOKEN").expect("DISCORD_BOT_TOKEN env var is not set!");
    let sched = JobScheduler::new()
        .await
        .expect("Couldn't create a job scheduler");
    let channel_id =
        std::env::var("DISCORD_CHANNEL_ID").expect("DISCORD_CHANNEL_ID env var is not set!");
    let channel_id: u64 = channel_id
        .parse()
        .expect("DISCORD_CHANNEL_ID can't be casted to number");
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;
    let handler = Handler::new(channel_id, sched);
    let mut client = Client::builder(&token, intents)
        .event_handler(handler)
        .await
        .expect("Unable to create a client");

    if let Err(err) = client.start().await {
        println!("{:?}", err)
    }
}
