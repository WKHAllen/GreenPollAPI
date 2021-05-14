SELECT * FROM app_user WHERE email = (
    SELECT email FROM password_reset WHERE id = $1
);
