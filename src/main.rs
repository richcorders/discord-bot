// SPDX-FileCopyrightText: 2024 winston <hey@winston.sh>
// SPDX-License-Identifier: AGPL-3.0-only

#![deny(clippy::nursery, clippy::pedantic, clippy::perf)]
#![allow(clippy::wildcard_imports)]
use std::collections::HashMap;
use std::env;

// for diesel dsl::*
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use parking_lot::Mutex;
use poise::serenity_prelude::{self as serenity, ActivityData, GatewayIntents, GuildId};
use sql::models::BotOptions;
use sql::options;

use crate::event_handler::event_handler;
mod commands;
mod event_handler;
mod sql;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

pub struct Data {
    db_pool: Pool<ConnectionManager<PgConnection>>,
    bot_options: Mutex<HashMap<GuildId, BotOptions>>,
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let token = env::var("DISCORD_TOKEN").expect("Missing DISCORD_TOKEN.");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");

    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let db_pool = sql::get_connection_pool(database_url).expect("Failed to connect to database.");
    sql::run_migrations(
        &mut db_pool
            .get()
            .expect("Failed to get pool connection for initial migrations."),
    )
    .expect("Failed to run migrations.");

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
                let conn = &mut db_pool.get().unwrap();
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                ctx.set_activity(Some(ActivityData::watching("dn")));
                let bot_options = options::get(conn).unwrap();

                Ok(Data {
                    db_pool,
                    bot_options: Mutex::new(bot_options),
                })
            })
        })
        .options(options)
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    client
        .expect("Failed to create serenity client.")
        .start()
        .await
        .expect("Failed to start serenity client.");
}
