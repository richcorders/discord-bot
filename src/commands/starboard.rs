// FIXME: remove once starboard has actual commands
#![allow(clippy::unused_async)]
use poise::serenity_prelude::ReactionType;

use crate::{Context, Error};

/// Manage starboard(s) for your server.
#[poise::command(slash_command, guild_only, subcommands("create", "remove"))]
#[allow(clippy::unused_async)]
pub async fn starboard(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// Create a new starboard for your server.
#[poise::command(slash_command, guild_only)]
pub async fn create(ctx: Context<'_>, emoji: ReactionType) -> Result<(), Error> {
    let conn = &mut ctx.data().db_pool.clone().get().unwrap();
    Ok(())
}

/// Create a new starboard for your server.
#[poise::command(slash_command, guild_only)]
pub async fn remove(
    ctx: Context<'_>,
    #[description = "The starboard to remove"] emoji: ReactionType,
) -> Result<(), Error> {
    Ok(())
}
