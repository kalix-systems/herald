SELECT EXISTS (
   SELECT 1 FROM userkeys WHERE key=$1
)
