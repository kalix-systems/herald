SELECT EXISTS (
    SELECT
        1
    FROM
        keys
    WHERE
        public_key = @public_key
    LIMIT 1
)
