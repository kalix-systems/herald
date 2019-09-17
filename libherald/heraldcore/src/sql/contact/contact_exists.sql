SELECT
  EXISTS (
    SELECT
      user_id
    FROM
      contacts
    WHERE
      user_id = ?
    LIMIT
      1
  )