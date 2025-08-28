-- Your SQL goes here
CREATE TABLE incomes (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    value REAL NOT NULL,
    date TEXT NOT NULL,
    name TEXT NOT NULL,
    category TEXT NOT NULL
);
