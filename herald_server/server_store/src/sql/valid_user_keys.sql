select
    key_creations.key
from
    key_creations
inner join
    userkeys
on
    key_creations.key = userkeys.key
where
    userkeys.user_id = $1 and
    key_creations.key not in (
        select
            key_deprecations.key
        from
            key_deprecations
    );
