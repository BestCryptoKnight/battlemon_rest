-- Add migration script here
CREATE TABLE sales
(
    id         BIGSERIAL PRIMARY KEY,
    prev_owner TEXT        NOT NULL,
    curr_owner TEXT        NOT NULL,
    token_id   TEXT        NOT NULL,
    price      DECIMAL     NOT NULL,
    date       timestamptz NOT NULL
);