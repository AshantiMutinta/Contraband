-- Your SQL goes here

CREATE TABLE IF NOT EXISTS items(
item_id INTEGER PRIMARY KEY  NOT NULL,
key TEXT NOT NULL,
value TEXT NOT NULL,
thread INTEGER NOT NULL,
FOREIGN KEY(thread) REFERENCES threads(thread_id)
);