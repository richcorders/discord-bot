CREATE TABLE starboarded_messages (
    message_id BIGINT PRIMARY KEY,
    starboard_id BIGINT NOT NULL,
    author_id BIGINT NOT NULL,
    react_count INTEGER NOT NULL
);