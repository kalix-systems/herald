use super::*;

pub async fn keys_of(mut store: Conn, req: keys_of::Req) -> Result<keys_of::Res, Error> {
    use keys_of::*;

    let mut map = Vec::with_capacity(req.0.len());

    for uid in req.0 {
        let meta = store.read_meta(&uid).await?;
        map.push((uid, meta));
    }

    Ok(Res(map))
}

pub async fn key_info(mut store: Conn, req: key_info::Req) -> Result<key_info::Res, Error> {
    use key_info::*;

    let mut map = Vec::with_capacity(req.0.len());

    for key in req.0 {
        let meta = store.read_key(key).await?;
        map.push((key, meta));
    }

    Ok(Res(map))
}

pub async fn keys_exist(mut store: Conn, req: keys_exist::Req) -> Result<keys_exist::Res, Error> {
    use keys_exist::*;

    let mut vec = Vec::with_capacity(req.0.len());

    for key in req.0 {
        vec.push(store.device_exists(&key).await?);
    }

    Ok(Res(vec))
}

pub async fn users_exist(mut store: Conn, req: users_exist::Req) -> Result<users_exist::Res, Error> {
    use users_exist::*;

    let mut vec = Vec::with_capacity(req.0.len());

    for uid in req.0 {
        vec.push(store.user_exists(&uid).await?);
    }

    Ok(Res(vec))
}
