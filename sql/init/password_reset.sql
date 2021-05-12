CREATE TABLE IF NOT EXISTS password_reset (
    id          CHAR(16)    NOT NULL,
    email       VARCHAR(63) NOT NULL,
    create_time TIMESTAMP   NOT NULL DEFAULT NOW(),

    PRIMARY KEY (id)
);
