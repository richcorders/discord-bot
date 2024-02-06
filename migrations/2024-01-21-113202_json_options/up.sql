DROP TABLE starboard_options;
DROP TABLE bot_options;
CREATE TABLE bot_options (
    "guild_id" BIGINT PRIMARY KEY,
    "prefix" TEXT NOT NULL DEFAULT '!',
    "starboard_options" JSONB NOT NULL DEFAULT '{}'
);