-- Add migration script here
create table bids
(
    id         varchar primary key,
    token_id   text    not null,
    expire_at  timestamptz,
    account_id text,
    price      decimal not null,
    create_at  timestamptz
)