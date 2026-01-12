use std::path::PathBuf;

use dotenvy::dotenv;
use poise::serenity_prelude as serenity;
use rand::Rng;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

struct Data {
    config_dir: PathBuf,

    pub counter: Mutex<u128>,
} // User data, which is stored and accessible in all command invocations
impl Data {
    
}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Displays your or another user's account creation date
#[poise::command(slash_command)]
async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}

/// Advances the counter
#[poise::command(slash_command)]
async fn counter(ctx: Context<'_>) -> Result<(), Error> {
    let data = ctx.data();
    let mut counter = data.counter.lock().await;
    *counter += 1;
    let response = format!("Counter: {}", counter);
    ctx.say(response).await?;
    Ok(())
}

/// Random number generator
#[poise::command(slash_command)]
async fn dice(ctx: Context<'_>, #[description = "Max number"] #[min = 1] eyes: Option<u32>) -> Result<(), Error> {
    let result = rand::rng().random_range(1..=eyes.unwrap_or(6));
    ctx.reply(format!("You rolled a {}!", result)).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = std::env::var("TOKEN").expect("missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::non_privileged()
        | serenity::GatewayIntents::GUILD_MESSAGES
        | serenity::GatewayIntents::GUILD_MEMBERS
        | serenity::GatewayIntents::MESSAGE_CONTENT;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![age(), counter(), dice()],
            event_handler: |ctx, event, _framework, data| {
                Box::pin(async move {
                    match event {
                        serenity::all::FullEvent::Ready { data_about_bot } => {
                            println!("Bot is ready as '{}'!", data_about_bot.user.name);
                        }

                        serenity::all::FullEvent::Message { new_message } => {
                            println!("Message: {}", new_message.content);
                            tokio::task::spawn_blocking(|| {
                                std::thread::sleep(std::time::Duration::from_secs(5));
                            })
                            .await?;
                            //println!("time's up")
                        }

                        _ => {}
                    }

                    Ok(())
                })
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    config_dir: "./botconfig/".into(),
                    counter: 0u128.into(),
                })
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}
