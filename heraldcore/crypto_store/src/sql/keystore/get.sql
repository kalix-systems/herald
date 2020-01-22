SELECT
    key
FROM
    keys
WHERE
    public_key = @public_key AND
    ix = @ix
LIMIT 1
