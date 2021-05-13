INSERT INTO poll
    (user_id, title, description)
VALUES
    ($1, $2, $3)
RETURNING *;
