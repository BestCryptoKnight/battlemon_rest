-- Add migration script here
CREATE TABLE sales
(
    id         uuid        NOT NULL,
    PRIMARY KEY (id),
    prev_owner TEXT        NOT NULL,
    curr_owner TEXT        NOT NULL,
    token_id   TEXT UNIQUE NOT NULL,
    price      DECIMAL     NOT NULL,
    date       timestamptz NOT NULL
);