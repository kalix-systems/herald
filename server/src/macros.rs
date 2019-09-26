#[macro_export]
macro_rules! mk_filter {
    ($this: expr, $f: ident) => {
        warp::path(stringify!($f))
            .and(warp::filters::body::concat())
            .map(move |b| {
                $this
                    .req_handler(b, $f)
                    .unwrap_or_else(|e| format!("{:?}", e).into())
            })
    };
}
