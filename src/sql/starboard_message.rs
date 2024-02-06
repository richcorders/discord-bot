use diesel::prelude::*;
use diesel::result::Error;

use super::models;
use super::schema::{self};
use crate::sql::models::StarboardedMessage;

pub fn upsert(
    conn: &mut PgConnection,
    message_id_val: i64,
    starboard_id_val: i64,
    author_id_val: i64,
    react_count_val: i32,
    manual_val: bool,
) -> Result<models::StarboardedMessage, Error> {
    use schema::starboarded_messages;
    use schema::starboarded_messages::dsl::*;

    let new = models::StarboardedMessage {
        message_id: message_id_val,
        starboard_id: starboard_id_val,
        author_id: author_id_val,
        react_count: react_count_val,
        manual: manual_val,
    };

    diesel::insert_into(starboarded_messages::table)
        .values(&new)
        .on_conflict(message_id)
        .do_update()
        .set(react_count.eq(react_count_val))
        .returning(models::StarboardedMessage::as_returning())
        .get_result(conn)
}

pub fn exists(conn: &mut PgConnection, message_id_val: i64) -> Result<bool, Error> {
    use schema::starboarded_messages::dsl::*;

    starboarded_messages
        .filter(message_id.eq(message_id_val))
        .first(conn)
        .optional()
        .map(|x: Option<StarboardedMessage>| x.is_some())
}
