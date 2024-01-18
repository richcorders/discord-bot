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
    pub id: i64,
    pub score: f64,
    pub time_stamp: NaiveDateTime,
}
