INSERT INTO poll_vote
    (user_id, poll_id, poll_option_id)
VALUES
    ($1, $2, $3)
RETURNING *;
