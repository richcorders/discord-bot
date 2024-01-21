CREATE TABLE degen_leaderboard (
    user_id BIGINT PRIMARY KEY,
    score FLOAT NOT NULL,
    time_stamp TIMESTAMP NOT NULL DEFAULT now()
);