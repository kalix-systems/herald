SELECT
  id,
  name,
  profile_picture,
  color,
  colorscheme,
  pairwise_conversation,
  kp
FROM
  config
  INNER JOIN contacts ON config.id = contacts.user_id
LIMIT
  1