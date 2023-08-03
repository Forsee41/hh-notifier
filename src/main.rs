use dotenv::dotenv;
use hh_notifier::{env_vars::EnvVars, handler::Handler};
use serenity::prelude::{Client, GatewayIntents};
use tokio_cron_scheduler::JobScheduler;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = std::env::var("DISCORD_BOT_TOKEN").expect("DISCORD_BOT_TOKEN env var is not set!");
    let sched = JobScheduler::new()
        .await
        .expect("Couldn't create a job scheduler");
    let env_vars = EnvVars::load().expect("Unable to load env vars");
    let config = env_vars.to_bot_config();
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;
    let handler = Handler::new(sched, config);
    let mut client = Client::builder(&token, intents)
        .event_handler(handler)
        .await
        .expect("Unable to create a client");

    if let Err(err) = client.start().await {
        println!("{:?}", err)
    }
}
