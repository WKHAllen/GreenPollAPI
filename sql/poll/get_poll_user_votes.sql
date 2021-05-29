SELECT
  app_user.id AS user_id,
  app_user.username AS username,
  poll_vote.poll_option_id AS poll_option_id,
  poll_option.value AS poll_option_value,
  poll_vote.vote_time AS vote_time
FROM app_user
JOIN poll_vote ON app_user.id = poll_vote.user_id
JOIN poll_option ON poll_vote.poll_option_id = poll_option.id
WHERE poll_vote.poll_id = $1;
