CREATE TABLE IF NOT EXISTS password_reset (
    id          CHAR(16)    NOT NULL DEFAULT SUBSTR(MD5(RANDOM()::TEXT), 0, 16),
    email       VARCHAR(63) NOT NULL,
    create_time TIMESTAMP   NOT NULL DEFAULT NOW(),

    PRIMARY KEY (id)
);
