INSERT INTO
  conversations(
    conversation_id,
    title,
    picture,
    color,
    pairwise,
    muted,
    last_active_ts,
    expiration_period,
    status
  )
VALUES(
  @conversation_id,
  @title,
  @picture,
  @color,
  @pairwise,
  @muted,
  @last_active_ts,
  @expiration_period,
  @status
)
