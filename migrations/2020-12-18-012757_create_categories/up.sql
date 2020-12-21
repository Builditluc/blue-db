CREATE TABLE categories
(
    id         TEXT      NOT NULL PRIMARY KEY,
    title      VARCHAR   NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
)