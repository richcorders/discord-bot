use std::fmt::Write;

use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::dsl::count;
use diesel::prelude::*;
use diesel::result::Error;
use poise::serenity_prelude::utils::*;

use super::{models, schema};

/// creates human-readable message stats
#[allow(clippy::cast_precision_loss)]
pub fn get_message_stats(conn: &mut PgConnection, since_days: Option<u32>) -> String {
    use schema::messages::dsl::*;

    let total: i64 = messages.count().get_result(conn).unwrap_or(0);

    let mut recent_timestamp: Option<DateTime<Utc>> = None;
    let recent: Option<i64> = since_days.map(|since| {
        recent_timestamp = Some(Utc::now() - chrono::Duration::days(since.into()));
        messages
            .filter(time_stamp.ge(recent_timestamp.unwrap().naive_utc()))
            .count()
            .get_result(conn)
            .unwrap_or(0)
    });

    let top_users = messages
        .group_by(author_id)
        .select((author_id, count(id)))
        .order_by(count(id).desc())
        .limit(10)
        .load::<(i64, i64)>(conn);

    let leaderboard = top_users.map(|top_users| {
        top_users
            .into_iter()
            .enumerate()
            .fold(String::new(), |mut acc, (i, (user_id, count))| {
                let _ = writeln!(
                    acc,
                    "{}. <@{}> - {} ({}%)",
                    i + 1,
                    user_id,
                    count,
                    (count as f64 / total as f64) * 100.0
                );
                acc
            })
    });

    let message = [
        format!("`dn` were recorded a total of {total} times.\n"),
        recent.map_or(String::new(), |recent| {
            format!(
                "({} times since {})\n",
                recent,
                FormattedTimestamp::new(
                    recent_timestamp.unwrap().into(),
                    Some(FormattedTimestampStyle::ShortDate)
                )
            )
        }),
        "\n**Leaderboard:**\n".to_string(),
        leaderboard.unwrap_or(String::new()),
    ];

    message.join("")
}

pub fn create(
    conn: &mut PgConnection,
    id: i64,
    content: &str,
    time_stamp: NaiveDateTime,
    author_id: i64,
) -> Result<models::Message, Error> {
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
}
