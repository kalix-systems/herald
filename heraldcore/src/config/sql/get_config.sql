SELECT
  id,
  name,
  profile_picture,
  color,
  colorscheme,
  pairwise_conversation,
  home_server,
  preferred_expiration
FROM
  config
  INNER JOIN users ON config.id = users.user_id
LIMIT
  1
