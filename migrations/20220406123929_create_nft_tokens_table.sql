CREATE TABLE nft_tokens
(
    id            BIGSERIAL PRIMARY KEY,
    owner_id      TEXT        NOT NULL,
    token_id      TEXT UNIQUE NOT NULL,
    title         TEXT,
    description   TEXT,
    media         TEXT        NOT NULL,
    media_hash    TEXT,
    copies        TEXT,
    issued_at     TEXT,
    expires_at    TEXT,
    model         JSONB       NOT NULL,
    db_created_at timestamptz NOT NULL
);
