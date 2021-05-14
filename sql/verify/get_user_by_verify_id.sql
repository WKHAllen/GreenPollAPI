SELECT * FROM app_user WHERE email = (
    SELECT email FROM verify WHERE id = $1
);
