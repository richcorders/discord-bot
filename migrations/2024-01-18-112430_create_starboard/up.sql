-- SPDX-FileCopyrightText: 2024 2024 winston <hey@winston.sh>
--
-- SPDX-License-Identifier: AGPL-3.0-only

CREATE TABLE starboarded_messages (
    "message_id" BIGINT PRIMARY KEY,
    "starboard_id" BIGINT NOT NULL,
    "author_id" BIGINT NOT NULL,
    "react_count" INTEGER NOT NULL,
    "manual" BOOLEAN NOT NULL
);