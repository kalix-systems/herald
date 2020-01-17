SELECT
    outer_ts,
    outer_signature,
    outer_signed_by,

    inner_ts,
    inner_signature,
    inner_signed_by,

    user_id TEXT
FROM
    sigchain_endorsements
WHERE
    user_id = @user_id
