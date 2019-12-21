select
    key_creations.key
from
    key_creations
inner join
    user_keys
on
    key_creations.key = userkeys.key
where
    user_keys.user_id <> $1 and
    key_creations.key not in (
        select
            key_deprecations.key
        from
            key_deprecations
    );
