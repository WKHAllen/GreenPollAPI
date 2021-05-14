INSERT INTO session
    (user_id)
VALUES
    ($1)
RETURNING *;
