-- Your SQL goes here

CREATE TABLE IF NOT EXISTS threads(
thread_id INTEGER PRIMARY KEY NOT NULL,
operation_name TEXT NOT NULL,
operation_text TEXT NOT NULL,
replies INTEGER NOT NULL,
last_updated TEXT NOT NULL
);