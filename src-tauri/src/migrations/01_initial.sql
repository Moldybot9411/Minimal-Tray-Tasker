CREATE TABLE trackers (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    amount INTEGER NOT NULL,
    progress INTEGER NOT NULL,
    completed BOOLEAN NOT NULL CHECK (completed IN (0, 1)),
    is_daily BOOLEAN NOT NULL CHECK (is_daily IN (0,1)),
    last_modified_at INTEGER NOT NULL
);