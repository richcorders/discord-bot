#![deny(clippy::nursery, clippy::pedantic, clippy::perf)]
#![allow(clippy::wildcard_imports)]
use std::collections::HashMap;
use std::env;

// for diesel dsl::*
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use parking_lot::Mutex;
use poise::serenity_prelude::{
    self as serenity, ActivityData, ChannelId, GatewayIntents, GuildId, ReactionType,
};

use crate::event_handler::event_handler;
mod commands;
mod event_handler;
mod sql;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[derive(Default, Clone, Copy)]
pub struct StarboardOptions {
    channel_id: ChannelId,
    threshold: u64,
}
// unsafe impl Send for StarboardOptions {}
// unsafe impl Sync for StarboardOptions {}
#[derive(Default, Clone)]
pub struct BotOptions {
    starboard_options: HashMap<GuildId, HashMap<ReactionType, StarboardOptions>>,
}
// unsafe impl Send for BotOptions {}
// unsafe impl Sync for BotOptions {}
pub struct Data {
    db_pool: Pool<ConnectionManager<PgConnection>>,
    bot_options: Mutex<BotOptions>,
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let token = env::var("DISCORD_TOKEN").expect("Missing DISCORD_TOKEN.");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");

    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let db_pool = sql::get_connection_pool(database_url).expect("Failed to connect to database.");
    sql::run_migrations(&mut db_pool.get().unwrap()).expect("Failed to run migrations.");

    let options = poise::FrameworkOptions {
        commands: vec![
            commands::degen_leaderboard(),
            commands::dn_stats(),
            commands::help(),
            commands::register(),
        ],
        event_handler: |ctx, event, _, data| {
            Box::pin(async move {
                event_handler(ctx, event, data).await;
                Ok(())
            })
        },
        ..Default::default()
    };

    let framework = poise::Framework::builder()
        .setup(move |ctx, _, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                ctx.set_activity(Some(ActivityData::watching("dn")));

                let mut dev_options = BotOptions {
                    starboard_options: HashMap::new(),
                };
                dev_options.starboard_options.insert(
                    840086859119460382.into(),
                    vec![(
                        ReactionType::Unicode("⭐".into()),
                        StarboardOptions {
                            channel_id: 1197472255295377470.into(),
                            threshold: 2,
                        },
                    )]
                    .into_iter()
                    .collect(),
                );

                Ok(Data {
                    db_pool,
                    bot_options: dev_options.into(),
                })
            })
        })
        .options(options)
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap();
}
