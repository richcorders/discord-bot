-- SPDX-FileCopyrightText: 2024 winston <hey@winston.sh>
--
-- SPDX-License-Identifier: AGPL-3.0-only

CREATE TABLE messages (
  "id" BIGINT PRIMARY KEY,
  "content" TEXT NOT NULL,
  "time_stamp" TIMESTAMP NOT NULL,
  "author_id" BIGINT NOT NULL
);