DELETE FROM poll_vote WHERE user_id = $1 AND poll_option_id = $2;
