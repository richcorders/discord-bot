// @generated automatically by Diesel CLI.

diesel::table! {
    messages (id) {
        id -> Int8,
        content -> Text,
        time_stamp -> Timestamp,
        author_id -> Int8,
    }
}
