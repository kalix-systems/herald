SELECT EXISTS (
   SELECT 1 FROM conversations WHERE conversation_id=$1
)
