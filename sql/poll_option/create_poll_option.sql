INSERT INTO poll_option
    (poll_id, value)
VALUES
    ($1, $2)
RETURNING *;
