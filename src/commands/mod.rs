mod degen_leaderboard;
mod dn_stats;
pub use degen_leaderboard::*;
pub use dn_stats::*;
use poise::serenity_prelude::*;

use crate::{Context, Error};

/// Simple helper command to register & sync all slash commands.
#[poise::command(prefix_command, hide_in_help, owners_only)]
pub async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}

/// Show help, optionally for a specific command.
#[poise::command(slash_command)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"] command: Option<String>,
) -> Result<(), Error> {
    let config = poise::builtins::HelpConfiguration {
        extra_text_at_bottom: "\
Use /help <command> for more info on a specific command.",
        ephemeral: true,
        ..Default::default()
    };
    poise::builtins::help(ctx, command.as_deref(), config).await?;
    Ok(())
}
