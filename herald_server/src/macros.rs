#[macro_export]
macro_rules! mk_filter {
    ($this: expr, $f: ident) => {
        warp::path(stringify!($f))
            .boxed()
            .and(::warp::filters::body::concat().boxed())
            .boxed()
            .and_then(move |b: ::warp::filters::body::FullBody| {
                async move {
                    let r1: Result<Vec<u8>, Error> = req_handler_store($this, b, $f).await;
                    let r2: Result<Vec<u8>, ::warp::reject::Rejection> =
                        r1.map_err(|e| ::warp::reject::custom(format!("{:?}", e)));
                    r2
                }
            })
            .boxed()
    };
}

#[macro_export]
macro_rules! push_filter {
    ($this: expr, $f: tt) => {
        warp::path(stringify!($f))
            .boxed()
            .and(::warp::filters::body::concat().boxed())
            .boxed()
            .and_then(move |b: ::warp::filters::body::FullBody| {
                async move {
                    let r1: Result<Vec<u8>, Error> = req_handler_async($this, b, State::$f).await;
                    let r2: Result<Vec<u8>, ::warp::reject::Rejection> =
                        r1.map_err(|e| ::warp::reject::custom(format!("{:?}", e)));
                    r2
                }
            })
            .boxed()
    };
}
