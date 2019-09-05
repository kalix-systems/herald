SELECT
  EXISTS (
    SELECT
      1
    FROM
      contacts
    WHERE
      user_id = ?
    LIMIT
      1
  )