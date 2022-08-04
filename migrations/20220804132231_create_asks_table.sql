-- Add migration script here
create table asks
(
    id          varchar primary key,
    token_id    text      not null,
    account_id  text      not null,
    approval_id bigserial not null,
    price       decimal   not null
)
