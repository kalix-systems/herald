INSERT INTO
  users(
    user_id,
    name,
    profile_picture,
    color,
    status,
    pairwise_conversation,
    user_type
  )
VALUES(@1, @2, @3, @4, @5, @6, @7)