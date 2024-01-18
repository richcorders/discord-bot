use poise::serenity_prelude::{self as serenity, FullEvent};

use crate::sql::message::create_message;
use crate::Data;

fn check_for_dn(message: &serenity::Message) -> bool {
    let keyword = "dn";
    let msg = clean_dn(&message.content);
    msg.split(' ').count() <= 6 && msg.contains(keyword)
}

/// removes common prefixes and suffixes from dn, lowercases dn
fn clean_dn(message: &str) -> String {
    message
        .to_lowercase()
        .replace("hbu", "")
        .replace("how about you", "")
}

pub fn handle_dn(event: &FullEvent, data: &Data) {
    match event {
        FullEvent::Message { new_message } => {
            if check_for_dn(new_message) {
                let pool = data.db_pool.clone();
                let conn = &mut pool.get().unwrap();
                let _ = create_message(
                    conn,
                    new_message.id.into(),
                    &new_message.content,
                    new_message.timestamp.naive_utc(),
                    new_message.author.id.into(),
                );
            }
        }
        FullEvent::MessageUpdate {
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
                    let _ = create_message(
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
}
