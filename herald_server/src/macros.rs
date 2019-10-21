#[macro_export]
macro_rules! mk_filter {
    ($this: expr, $f: ident) => {
        warp::path(stringify!($f))
            .and(warp::filters::body::concat())
            .map(move |b| {
                async move {
                    $this
                        .req_handler(b, $f)
                        .unwrap_or_else(|e| format!("{:?}", e).into())
                }
            })
    };
}

#[macro_export]
macro_rules! push_filter {
    ($this: expr, $f: tt) => {
        warp::path(stringify!($f))
            .and(warp::filters::body::concat())
            .and_then(move |b| {
                async move {
                    $this
                        .req_handler_async(b, State::$f)
                        .await
                        // I kinda hate this but warp forces it...
                        .map_err(|e| warp::reject::custom(format!("{:?}", e)))
                }
            })
    };
}
