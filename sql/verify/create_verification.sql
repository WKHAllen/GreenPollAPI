INSERT INTO verify
    (email)
VALUES
    ($1)
RETURNING *;
