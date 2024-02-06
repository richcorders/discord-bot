use std::collections::HashMap;

use diesel::prelude::*;
use poise::serenity_prelude::GuildId;

use super::models::BotOptions;

/// get the options for all guilds
pub fn get(conn: &mut PgConnection) -> Result<HashMap<GuildId, BotOptions>, diesel::result::Error> {
    use super::schema::bot_options::dsl::*;

    bot_options
        .select(BotOptions::as_select())
        .load(conn)
        .map(|options| {
            #[allow(clippy::cast_sign_loss)]
            let options: HashMap<GuildId, BotOptions> = options
                .into_iter()
                .map(|option| ((option.guild_id as u64).into(), option))
                .collect();
            options
        })
}

/// update the options for a guild
pub fn upsert(
    conn: &mut PgConnection,
    guild_options: &BotOptions,
) -> Result<BotOptions, diesel::result::Error> {
    use super::schema::bot_options::dsl::*;

    diesel::insert_into(bot_options)
        .values(guild_options)
        .on_conflict(guild_id)
        .do_update()
        .set(guild_options)
        .returning(BotOptions::as_returning())
        .get_result(conn)
}
