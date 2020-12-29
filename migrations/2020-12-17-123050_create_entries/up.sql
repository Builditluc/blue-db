CREATE TABLE entries
(
    id          TEXT      NOT NULL PRIMARY KEY,
    category_id TEXT      NOT NULL,
    title       VARCHAR   NOT NULL,
    body        TEXT      NOT NULL,
    created_at  TIMESTAMP NOT NULL,
    updated_at  TIMESTAMP NOT NULL,

    FOREIGN KEY (category_id) REFERENCES categories (id) ON DELETE CASCADE
)