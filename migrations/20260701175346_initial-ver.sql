-- Add migration script here
CREATE TABLE tracks (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    artist TEXT NOT NULL
);

CREATE TABLE users (
    id INTEGER PRIMARY KEY,
    username TEXT NOT NULL,
    password TEXT NOT NULL 
);