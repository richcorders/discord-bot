#![deny(clippy::nursery, clippy::pedantic, clippy::perf)]
#![allow(clippy::wildcard_imports)] // for diesel dsl::*
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use event_handler::event_handler;
use poise::serenity_prelude::{self as serenity, ActivityData};
mod commands;
mod dn;
mod event_handler;
mod sql;

struct Data {
    db_pool: Pool<ConnectionManager<PgConnection>>,
}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
    let db_pool = sql::get_connection_pool().expect("Failed to connect to database");
    sql::run_migrations(&mut db_pool.get().unwrap()).expect("Failed to run migrations");

    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::degen_leaderboard(),
                commands::dn_stats(),
                commands::help(),
                commands::register(),
            ],
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler(ctx, event, framework, data))
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                ctx.set_activity(Some(ActivityData::watching("dn")));
                Ok(Data { db_pool })
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}
