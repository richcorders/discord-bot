use poise::serenity_prelude::CreateEmbed;
use poise::CreateReply;

use crate::sql::message::get_message_stats;
use crate::{Context, Error};

/// Displays deez stats
#[poise::command(slash_command)]
pub async fn dn_stats(
    ctx: Context<'_>,
    #[description = "Get stats for the last n days"]
    #[rename = "since"]
    since_days: Option<u32>,
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
