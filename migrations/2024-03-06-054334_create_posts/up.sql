-- Your SQL goes here
CREATE TABLE crate_infos (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    path TEXT NOT NULL,
    result TEXT NOT NULL
);