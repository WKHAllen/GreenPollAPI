SELECT * from poll WHERE id = (
    SELECT poll_id FROM poll_vote WHERE id = $1
);
