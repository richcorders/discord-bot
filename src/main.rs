use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use event_handler::event_handler;
use poise::serenity_prelude::{self as serenity, ActivityData, CreateEmbed};
use poise::CreateReply;
use sql::message::get_message_stats;
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
    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![dn_stats()],
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler(ctx, event, framework, data))
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                ctx.set_activity(Some(ActivityData::watching("dn")));
                Ok(Data {
                    db_pool: sql::get_connection_pool()?,
                })
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}

/// Displays deez stats
#[poise::command(slash_command, prefix_command)]
async fn dn_stats(
    ctx: Context<'_>,
    #[description = "Get stats for the last n days"] since_days: Option<u32>,
) -> Result<(), Error> {
    let conn = &mut ctx.data().db_pool.clone().get().unwrap();

    let description = get_message_stats(conn, since_days);

    let reply = CreateReply::default().embed(
        CreateEmbed::default()
            .title("deez stats")
            .description(description),
    );
    ctx.send(reply).await?;
    Ok(())
}
