select distinct
    sigchain.key
from
    sigchain
inner join
    userkeys
on
    sigchain.key = userkeys.key
where
    userkeys.user_id = $1 and
    sigchain.key not in (
        select
            sigchain.key
        from
            sigchain
        where
            sigchain.is_creation = false
    );
