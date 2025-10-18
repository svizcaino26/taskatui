CREATE TABLE IF NOT EXISTS tasks (
    id INTEGER PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT,
    follow_up_date DATE,
    last_update DATE,
    completed BOOLEAN DEFAULT FALSE NOT NULL
);
