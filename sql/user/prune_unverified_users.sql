DELETE FROM app_user WHERE verified = FALSE AND EXTRACT(EPOCH FROM NOW() - join_time) >= 3600;
