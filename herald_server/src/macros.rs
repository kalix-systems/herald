#[macro_export]
macro_rules! mk_filter {
    ($this: expr, $f: ident) => {
        warp::path(stringify!($f))
            .and(::warp::filters::body::concat())
            .and_then(move |b: ::warp::filters::body::FullBody| {
                async move {
                    let r1: Result<Vec<u8>, Error> = $this.req_handler_store(b, $f).await;
                    let r2: Result<Vec<u8>, ::warp::reject::Rejection> =
                        r1.map_err(|e| ::warp::reject::custom(format!("{:?}", e)));
                    r2
                }
            })
    };
}

#[macro_export]
macro_rules! push_filter {
    ($this: expr, $f: tt) => {
        warp::path(stringify!($f))
            .and(::warp::filters::body::concat())
            .and_then(move |b: ::warp::filters::body::FullBody| {
                async move {
                    let r1: Result<Vec<u8>, Error> = $this.req_handler_async(b, State::$f).await;
                    let r2: Result<Vec<u8>, ::warp::reject::Rejection> =
                        r1.map_err(|e| ::warp::reject::custom(format!("{:?}", e)));
                    r2
                }
            })
    };
}
