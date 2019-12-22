SELECT EXISTS (
   SELECT 1 FROM conversation_members WHERE conversation_id=$1 LIMIT 1
)
