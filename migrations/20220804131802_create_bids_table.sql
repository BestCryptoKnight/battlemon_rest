-- Add migration script here
create table bids
(
    id         varchar primary key,
    token_id   text        not null,
    account_id text        not null,
    expire_at  timestamptz,
    create_at  timestamptz not null,
    price      decimal     not null
)