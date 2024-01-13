use poise::serenity_prelude::{self as serenity};

use crate::dn::check_for_dn;
use crate::sql::message::create_message;
use crate::{Data, Error};

pub async fn event_handler(
    _ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::Message { new_message } => {
            if check_for_dn(new_message) {
                let pool = data.db_pool.clone();
                let conn = &mut pool.get().unwrap();
                create_message(
                    conn,
                    new_message.id.into(),
                    &new_message.content,
                    new_message.timestamp.naive_utc(),
                    new_message.author.id.into(),
                );
            }
        }
        serenity::FullEvent::MessageUpdate {
            old_if_available,
            new,
            event: _,
        } => {
            if let (Some(old), Some(new)) = (old_if_available, new) {
                let old_contains = check_for_dn(old);
                let new_contains = check_for_dn(new);

                // only care about new messages inserting the message after the fact
                if !old_contains && new_contains {
                    let pool = data.db_pool.clone();
                    let conn = &mut pool.get().unwrap();
                    create_message(
                        conn,
                        new.id.into(),
                        &new.content,
                        new.timestamp.naive_utc(),
                        new.author.id.into(),
                    );
                }
            }
        }
        _ => {}
    }
    Ok(())
}
