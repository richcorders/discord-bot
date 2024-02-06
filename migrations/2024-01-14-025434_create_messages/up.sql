CREATE TABLE messages (
  "id" BIGINT PRIMARY KEY,
  "content" TEXT NOT NULL,
  "time_stamp" TIMESTAMP NOT NULL,
  "author_id" BIGINT NOT NULL
);