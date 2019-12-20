SELECT
    sealed_key
FROM
    prekeys
WHERE
    signing_key = $1
