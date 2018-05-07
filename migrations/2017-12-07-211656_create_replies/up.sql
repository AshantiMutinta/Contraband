-- Your SQL goes here

CREATE TABLE IF NOT EXISTS replies(
reply_id INTEGER PRIMARY KEY NOT NULL,
reply_comment TEXT NOT NULL,
last_updated TEXT NOT NULL,
thread INTEGER NOT NULL,
FOREIGN KEY(thread) REFERENCES threads(thread_id)
);