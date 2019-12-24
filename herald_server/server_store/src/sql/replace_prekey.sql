UPDATE
    prekeys
SET
    key = $1,
    signed_by = $2,
    signature = $3,
    ts = $4
WHERE
    key = $5
