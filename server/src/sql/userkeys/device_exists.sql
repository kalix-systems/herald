SELECT
  EXISTS(
    SELECT
      1
    FROM
      userkeys
    WHERE
      signing_key = $1
  )
