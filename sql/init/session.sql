CREATE TABLE IF NOT EXISTS session (
    id          CHAR(16)  NOT NULL DEFAULT SUBSTR(MD5(RANDOM()::TEXT), 0, 16),
    user_id     SERIAL    NOT NULL,
    create_time TIMESTAMP NOT NULL DEFAULT NOW(),

    PRIMARY KEY (id),

    CONSTRAINT fk_session_user
        FOREIGN KEY (user_id)
            REFERENCES app_user(id)
                ON DELETE CASCADE
);
