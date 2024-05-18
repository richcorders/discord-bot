-- SPDX-FileCopyrightText: 2024 winston <hey@winston.sh>
--
-- SPDX-License-Identifier: AGPL-3.0-only

CREATE TABLE degen_leaderboard (
    "user_id" BIGINT PRIMARY KEY,
    "score" FLOAT NOT NULL,
    "time_stamp" TIMESTAMP NOT NULL DEFAULT now()
);