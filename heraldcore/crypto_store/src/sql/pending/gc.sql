DELETE FROM
    payloads
WHERE
    payload_id
NOT IN (
    SELECT
        pending_payload_id
    FROM
        pending
);
