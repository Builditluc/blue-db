CREATE TABLE entries
(
    id        INTEGER   NOT NULL PRIMARY KEY,
    title     VARCHAR   NOT NULL,
    body      TEXT      NOT NULL,
    timestamp TIMESTAMP NOT NULL,
    entry_id  TEXT      NOT NULL
)