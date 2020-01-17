SELECT
    key
FROM
(
    (SELECT
       inner_signed_by as key
    FROM
       sigchain_endorsements
    )
UNION
    (
    SELECT
        key
    FROM
        sigchain_genesis
    )
)
WHERE
    key NOT IN
(
    SELECT
        key
    FROM
        sigchain_deprecations
)
