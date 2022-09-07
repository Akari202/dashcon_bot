use std::env;
use serenity::async_trait;
use serenity::framework::standard::macros::group;
use serenity::framework::standard::StandardFramework;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use tracing::{info, error};
use uwuifier::{uwuify_sse, round_up16};

const ALLOWED_CHANNELS: [u64; 2] = [
    871613480082485268,
    1016186008561270895
];

struct Handler;

#[group]
struct General;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} initalised", ready.user.name);
    }

    async fn message(&self, ctx: Context, message: Message) {
        if !message.author.bot {
            if ALLOWED_CHANNELS.contains(&message.channel_id.0) {
                if message.attachments.len() > 0 {
                    info!("{} had an attachment and was not modified", message.link());
                    return
                }
                let uwued = uwu_str(&message.content);
                if let Err(why) = message.delete(&ctx.http).await {
                    error!("Error deleting message: {}", why);
                }
                if let Err(why) = message.channel_id.send_message(&ctx.http, |m| {
                    m.embed(
                        |e| {
                            e.description(&uwued)
                                .author(
                                    |a| {
                                        a.name(&message.author.name)
                                            .icon_url(&message.author.avatar_url().unwrap())
                                    })
                        })
                }).await {
                    error!("Error sending message: {}", why);
                }
                info!(
                    "{} was replaced with: {}",
                    message.link(),
                    uwued
                );
            }
        }
    }
}

fn uwu_str(content: &String) -> String {
    let bytes = content.as_bytes();
    let mut temp1 = vec![0u8; round_up16(bytes.len()) * 16];
    let mut temp2 = vec![0u8; round_up16(bytes.len()) * 16];
    let res = uwuify_sse(bytes, &mut temp1, &mut temp2);
    std::str::from_utf8(res).unwrap().to_string()
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();
    let token = env::var("DISCORD_BOT_TOKEN")
        .expect("No bot token found");
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~"))
        .group(&GENERAL_GROUP);
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Err creating client");
    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
    ()
}
