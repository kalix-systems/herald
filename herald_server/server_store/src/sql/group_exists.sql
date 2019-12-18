SELECT EXISTS (
   SELECT 1 FROM conversations WHERE key=$1
)
