SELECT
  EXISTS (
    SELECT
      1
    FROM
      contacts
    WHERE
      id = ?
    LIMIT
      1
  )