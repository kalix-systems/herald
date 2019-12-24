DELETE FROM
    pushes
WHERE
    push_id NOT IN (
        SELECT push_id FROM pending
)
