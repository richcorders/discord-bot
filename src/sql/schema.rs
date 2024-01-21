// @generated automatically by Diesel CLI.

diesel::table! {
    bot_options (guild_id) {
        guild_id -> Int8,
        prefix -> Text,
    }
}

diesel::table! {
    degen_leaderboard (user_id) {
        user_id -> Int8,
        score -> Float8,
        time_stamp -> Timestamp,
    }
}

diesel::table! {
    messages (id) {
        id -> Int8,
        content -> Text,
        time_stamp -> Timestamp,
        author_id -> Int8,
    }
}

diesel::table! {
    starboard_options (guild_id) {
        guild_id -> Int8,
        channel_id -> Int8,
        emoji -> Text,
        threshold -> Int4,
    }
}

diesel::table! {
    starboarded_messages (message_id) {
        message_id -> Int8,
        starboard_id -> Int8,
        author_id -> Int8,
        react_count -> Int4,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    bot_options,
    degen_leaderboard,
    messages,
    starboard_options,
    starboarded_messages,
);
