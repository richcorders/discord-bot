use chrono::NaiveDateTime;
use diesel::prelude::*;

use super::{models, schema};

pub fn get_message_count(conn: &mut PgConnection, since: NaiveDateTime) -> i64 {
    use schema::messages::dsl::*;

    messages
        .filter(time_stamp.ge(since))
        .count()
        .get_result(conn)
        .expect("Error getting message count")
}

pub fn create_message(
    conn: &mut PgConnection,
    id: i64,
    content: &str,
    time_stamp: NaiveDateTime,
    author_id: i64,
) -> models::Message {
    use schema::messages;

    let new_message = models::Message {
        id,
        content: content.to_owned(),
        time_stamp,
        author_id,
    };

    diesel::insert_into(messages::table)
        .values(&new_message)
        .returning(models::Message::as_returning())
        .get_result(conn)
        .expect("Error saving new message")
}
