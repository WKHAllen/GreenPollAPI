INSERT INTO password_reset
    (email)
VALUES
    ($1)
RETURNING *;
