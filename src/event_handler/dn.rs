// SPDX-FileCopyrightText: 2024 winston <hey@winston.sh>
// SPDX-License-Identifier: AGPL-3.0-only

use poise::serenity_prelude::{Context, FullEvent, Message, ReactionType};

use crate::sql::message;
use crate::Data;

const KEYWORDS: [&str; 2] = ["dn", "deez"];

fn check_for_dn(message: &Message) -> bool {
    let msg = clean_dn(&message.content);
    msg.split(' ').count() <= 6 && KEYWORDS.contains(&msg.as_str())
}

fn clean_dn(message: &str) -> String {
    message
        .to_lowercase()
        .replace("hbu", "")
        .replace("how about you", "")
}

async fn record_dn(ctx: &Context, msg: &Message, data: &Data) {
    let pool = data.db_pool.clone();
    let conn = &mut pool.get().unwrap();

    let _ = msg
        .react(
            ctx,
            ReactionType::Custom {
                animated: false,
                id: 1_197_448_146_566_004_756.into(),
                name: Some("pepenotes".into()),
            },
        )
        .await;

    let _ = message::create(
        conn,
        msg.id.into(),
        &msg.content,
        msg.timestamp.naive_utc(),
        msg.author.id.into(),
    );
}

pub async fn handle(ctx: &Context, event: &FullEvent, data: &Data) {
    match event {
        FullEvent::Message { new_message } => {
            if check_for_dn(new_message) {
                record_dn(ctx, new_message, data).await;
            }
        }
        FullEvent::MessageUpdate {
            old_if_available,
            new,
            ..
        } => {
            if let (Some(old), Some(new)) = (old_if_available, new) {
                let old_contains = check_for_dn(old);
                let new_contains = check_for_dn(new);

                // only care about new messages inserting the keyword after the fact
                if !old_contains && new_contains {
                    record_dn(ctx, new, data).await;
                }
            }
        }
        _ => {}
    }
}
