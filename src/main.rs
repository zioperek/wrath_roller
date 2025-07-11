mod commands;
mod utils;

use commands::general::{Data, age, skill_check};
use dotenv::dotenv;
use poise::serenity_prelude as serenity;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = std::env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN must be set in .env file.");
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![age(), skill_check()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                // poise::builtins::register_globally(ctx, &framework.options().commands).await?; // globaly register commands in discord
                let guild_id = serenity::GuildId::new(
                    std::env::var("DISCORD_SERVER")
                        .expect("DISCORD_SERVER must be set in .env file.")
                        .parse()
                        .unwrap(),
                ); // Faster register for single Server
                poise::builtins::register_in_guild(ctx, &framework.options().commands, guild_id)
                    .await?;

                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}
