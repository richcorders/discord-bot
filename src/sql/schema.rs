// @generated automatically by Diesel CLI.

diesel::table! {
    degen_leaderboard (id) {
        id -> Int8,
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

diesel::allow_tables_to_appear_in_same_query!(degen_leaderboard, messages,);
