CREATE TABLE IF NOT EXISTS poll_option (
    id      SERIAL       NOT NULL,
    poll_id SERIAL       NOT NULL,
    value   VARCHAR(255) NOT NULL,

    PRIMARY KEY (id),

    CONSTRAINT fk_poll_option_poll
        FOREIGN KEY (poll_id)
            REFERENCES poll(id)
                ON DELETE CASCADE
);
