DELETE FROM
    pending
WHERE
    pending_payload_id = @id AND
    recipient = @recipient
