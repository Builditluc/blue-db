CREATE TABLE entries
(
    id          SERIAL    NOT NULL PRIMARY KEY,
    entry_id    TEXT      NOT NULL,
    category_id SERIAL    NOT NULL,
    title       VARCHAR   NOT NULL,
    body        TEXT      NOT NULL,
    created_at  TIMESTAMP NOT NULL,
    updated_at  TIMESTAMP NOT NULL,

    FOREIGN KEY (category_id) REFERENCES categories (id) ON DELETE CASCADE
)