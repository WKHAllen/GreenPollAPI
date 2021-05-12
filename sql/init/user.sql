CREATE TABLE IF NOT EXISTS app_user (
    id        SERIAL       NOT NULL,
    username  VARCHAR(63)  NOT NULL,
    email     VARCHAR(63)  NOT NULL,
    password  VARCHAR(255) NOT NULL,
    verified  BOOLEAN      NOT NULL DEFAULT FALSE,
    join_time TIMESTAMP    NOT NULL DEFAULT NOW(),

    PRIMARY KEY (id)
);
