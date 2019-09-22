SELECT
  EXISTS(
    SELECT
      1
    FROM
      userkeys
    WHERE
      user_id = $1
  )
