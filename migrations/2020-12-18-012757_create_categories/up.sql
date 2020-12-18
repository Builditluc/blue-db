CREATE TABLE categories
(
    id         SERIAL    NOT NULL PRIMARY KEY,
    title      TEXT      NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
)