DELETE FROM Session
WHERE user_id = $1 AND id NOT IN (
    SELECT id FROM Session
    WHERE user_id = $1
    ORDER BY create_time DESC
    LIMIT $2
);
