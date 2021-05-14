CREATE TABLE IF NOT EXISTS poll (
    id          SERIAL        NOT NULL,
    user_id     SERIAL        NOT NULL,
    title       VARCHAR(255)  NOT NULL,
    description VARCHAR(1023) NOT NULL,
    create_time TIMESTAMP     NOT NULL DEFAULT NOW(),

    PRIMARY KEY (id),

    CONSTRAINT fk_poll_user
        FOREIGN KEY (user_id)
            REFERENCES app_user(id)
                ON DELETE CASCADE
);
