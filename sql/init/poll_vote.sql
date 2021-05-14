CREATE TABLE IF NOT EXISTS poll_vote (
    id             SERIAL    NOT NULL,
    user_id        SERIAL    NOT NULL,
    poll_id        SERIAL    NOT NULL,
    poll_option_id SERIAL    NOT NULL,
    vote_time      TIMESTAMP NOT NULL DEFAULT NOW(),

    PRIMARY KEY (id),

    CONSTRAINT fk_poll_vote_user
        FOREIGN KEY (user_id)
            REFERENCES app_user(id)
                ON DELETE CASCADE,

    CONSTRAINT fk_poll_vote_poll
        FOREIGN KEY (poll_id)
            REFERENCES poll(id)
                ON DELETE CASCADE,

    CONSTRAINT fk_poll_vote_poll_option
        FOREIGN KEY (poll_option_id)
            REFERENCES poll_option(id)
                ON DELETE CASCADE
);
