INSERT OR IGNORE INTO keys(
    key,
    public_key,
    ix
)
VALUES(@key, @public_key, @ix)
