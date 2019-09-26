#[macro_export]
macro_rules! mk_filter {
    ($this: ident, $f: ident) => {
        warp::path(stringify!($f))
            .and(body::concat())
            .map(move |b| {
                $this
                    .req_handler(b, $f)
                    .map_err(|e| warp::reject::custom(format!("{:?}", e)))
            })
    };
}
