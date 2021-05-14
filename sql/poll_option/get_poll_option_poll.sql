SELECT * from poll WHERE id = (
    SELECT poll_id FROM poll_option WHERE id = $1
);
