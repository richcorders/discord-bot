use std::collections::HashMap;
use std::sync::Mutex;

use diesel::prelude::*;
use poise::serenity_prelude::ReactionType;

use super::models::{BotOptions as BotOptionsDB, StarboardOptions as StarboardOptionsDB};
use crate::{BotOptions, StarboardOptions};

pub fn get(conn: &mut PgConnection) -> Result<Mutex<BotOptions>, diesel::result::Error> {
    super::schema::bot_options::table
        .select(BotOptionsDB::as_select())
        .load(conn)
        .map(|options| {
            let mut starboard_options = HashMap::new();
            for bot_option in options {
                let mut starboard_options_for_guild = HashMap::new();
                let starboard_options_db = super::schema::starboard_options::table
                    .filter(super::schema::starboard_options::guild_id.eq(bot_option.guild_id))
                    .load::<StarboardOptionsDB>(conn)
                    .expect("Failed to load starboard options");
                for starboard_option in starboard_options_db {
                    starboard_options_for_guild.insert(
                        ReactionType::Unicode(starboard_option.emoji),
                        StarboardOptions {
                            channel_id: (starboard_option.channel_id as u64).into(),
                            threshold: starboard_option.threshold as u64,
                        },
                    );
                }
                starboard_options.insert(
                    (bot_option.guild_id as u64).into(),
                    starboard_options_for_guild,
                );
            }
            Mutex::new(BotOptions { starboard_options })
        })
}
