-- Add up migration script here
CREATE TABLE if not exists todos
(
    id        integer primary key,
    text      varchar(500),
    completed boolean
)