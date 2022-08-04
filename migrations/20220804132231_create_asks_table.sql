-- Add migration script here
create table asks
(
    id         varchar primary key,
    token_id   text    not null,
    expire_at  timestamptz,
    account_id text,
    approve_id bigserial,
    price      decimal not null
)
