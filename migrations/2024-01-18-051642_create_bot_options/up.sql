CREATE TABLE bot_options (
    "guild_id" BIGINT PRIMARY KEY,
    "prefix" TEXT NOT NULL DEFAULT '!'
);
CREATE TABLE starboard_options (
    "guild_id" BIGINT PRIMARY KEY,
    "channel_id" BIGINT NOT NULL,
    "emoji" TEXT NOT NULL,
    "threshold" INTEGER NOT NULL,
    UNIQUE ("channel_id", "emoji")
);