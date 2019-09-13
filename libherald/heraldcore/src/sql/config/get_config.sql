SELECT
  id,
  name,
  profile_picture,
  color,
  colorscheme
FROM
  config
  INNER JOIN contacts ON config.id = contacts.user_id
LIMIT
  1