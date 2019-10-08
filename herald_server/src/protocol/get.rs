use super::*;

pub fn keys_of(store: &mut Conn, req: keys_of::Req) -> Result<keys_of::Res, Error> {
    use keys_of::*;

    let mut map = Vec::with_capacity(req.0.len());

    for uid in req.0 {
        let meta = store.read_meta(&uid)?;
        map.push((uid, meta));
    }

    Ok(Res(map))
}

pub fn key_info(store: &mut Conn, req: key_info::Req) -> Result<key_info::Res, Error> {
    use key_info::*;

    let mut map = Vec::with_capacity(req.0.len());

    for key in req.0 {
        let meta = store.read_key(key)?;
        map.push((key, meta));
    }

    Ok(Res(map))
}

pub fn keys_exist(store: &mut Conn, req: keys_exist::Req) -> Result<keys_exist::Res, Error> {
    use keys_exist::*;

    let mut vec = Vec::with_capacity(req.0.len());

    for key in req.0 {
        vec.push(store.device_exists(&key)?);
    }

    Ok(Res(vec))
}

pub fn users_exist(store: &mut Conn, req: users_exist::Req) -> Result<users_exist::Res, Error> {
    use users_exist::*;

    let mut vec = Vec::with_capacity(req.0.len());

    for uid in req.0 {
        vec.push(store.user_exists(&uid)?);
    }

    Ok(Res(vec))
}
