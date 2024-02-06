use std::collections::HashMap;

use chrono::NaiveDateTime;
use diesel::prelude::*;
use poise::serenity_prelude::{self as serenity, ChannelId};
use serde::{Deserialize, Serialize};

use super::schema;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = schema::messages)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Message {
    pub id: i64,
    pub content: String,
    pub time_stamp: NaiveDateTime,
    pub author_id: i64,
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = schema::degen_leaderboard)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DegenLeaderboard {
    pub user_id: i64,
    pub score: f64,
    pub time_stamp: NaiveDateTime,
}

#[derive(Queryable, Selectable, Insertable, AsChangeset, Clone)]
#[diesel(table_name = schema::bot_options)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct BotOptions {
    pub guild_id: i64,
    pub prefix: String,
    pub starboard_options: diesel_json::Json<HashMap<serenity::ReactionType, StarboardPerEmoji>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StarboardPerEmoji {
    pub channel_id: ChannelId,
    pub threshold: u32,
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = schema::starboarded_messages)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct StarboardedMessage {
    pub message_id: i64,
    pub starboard_id: i64,
    pub author_id: i64,
    pub react_count: i32,
    pub manual: bool,
}
