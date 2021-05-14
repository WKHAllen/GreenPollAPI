SELECT * FROM app_user WHERE id = (
    SELECT user_id FROM session WHERE id = $1
);
