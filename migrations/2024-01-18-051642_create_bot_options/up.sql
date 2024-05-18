-- SPDX-FileCopyrightText: 2024 winston <hey@winston.sh>
--
-- SPDX-License-Identifier: AGPL-3.0-only

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