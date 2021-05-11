INSERT INTO app_user
    (email, password)
VALUES
    ($1, $2)
RETURNING *;
