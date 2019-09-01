UPDATE messages
SET status = @3, 
WHERE conversationId = @1, row = @2;