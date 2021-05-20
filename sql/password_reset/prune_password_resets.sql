DELETE FROM password_reset WHERE EXTRACT(EPOCH FROM NOW() - create_time) >= 3600;
