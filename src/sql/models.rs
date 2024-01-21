use chrono::NaiveDateTime;
use diesel::prelude::*;

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

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = schema::bot_options)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct BotOptions {
    pub guild_id: i64,
    pub prefix: String,
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = schema::starboard_options)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct StarboardOptions {
    pub guild_id: i64,
    pub channel_id: i64,
    pub emoji: String,
    pub threshold: i32,
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = schema::starboarded_messages)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct StarboardedMessage {
    pub message_id: i64,
    pub starboard_id: i64,
    pub author_id: i64,
    pub react_count: i32,
}
