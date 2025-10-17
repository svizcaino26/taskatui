CREATE TABLE IF NOT EXISTS tasks (
    id INTEGER PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT,
    follow_up_date TEXT,
    last_update TEXT,
    completed INTEGER DEFAULT 0
);
