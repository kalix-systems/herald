use super::*;
use warp::{filters::ws, Filter};

macro_rules! mk_filter {
    ($this: expr, $f: ident) => {
        warp::path(stringify!($f))
            .and(::warp::filters::body::concat())
            .and_then(move |b: ::warp::filters::body::FullBody| {
                async move {
                    let r1: Result<Vec<u8>, Error> = req_handler_store($this, b, $f).await;
                    let r2: Result<Vec<u8>, ::warp::reject::Rejection> =
                        r1.map_err(|e| ::warp::reject::custom(format!("{:?}", e)));
                    r2
                }
            })
    };
}

macro_rules! push_filter {
    ($this: expr, $f: tt) => {
        warp::path(stringify!($f))
            .and(::warp::filters::body::concat())
            .and_then(move |b: ::warp::filters::body::FullBody| {
                async move {
                    let r1: Result<Vec<u8>, Error> = req_handler_async($this, b, State::$f).await;
                    let r2: Result<Vec<u8>, ::warp::reject::Rejection> =
                        r1.map_err(|e| ::warp::reject::custom(format!("{:?}", e)));
                    r2
                }
            })
    };
}

pub async fn serve(
    state: &'static State,
    port: u16,
) {
    use warp::filters::method;
    let route_get = {
        use get::*;

        warp::path("echo")
            .boxed()
            .and(warp::filters::body::concat().boxed())
            .boxed()
            .map(|b: warp::body::FullBody| b.bytes().to_vec())
            .boxed()
            .or(mk_filter!(state, keys_of))
            .boxed()
            .or(mk_filter!(state, key_info))
            .boxed()
            .or(mk_filter!(state, keys_exist))
            .boxed()
            .or(mk_filter!(state, users_exist))
            .boxed()
    };
    let route_post = {
        use post::*;
        mk_filter!(state, register)
            .or(mk_filter!(state, new_key))
            .boxed()
            .or(mk_filter!(state, dep_key))
            .boxed()
            .or(push_filter!(state, push_users))
            .boxed()
            .or(push_filter!(state, push_devices))
            .boxed()
    };

    let routes = method::get2()
        .boxed()
        .and(route_get)
        .boxed()
        .or(method::post2().boxed().and(route_post).boxed())
        .boxed()
        .or(warp::path("login")
            .boxed()
            .and(ws::ws2().boxed())
            .boxed()
            .map(move |w: ws::Ws2| {
                w.on_upgrade(move |w: ws::WebSocket| {
                    async move {
                        state.handle_login(w).await.unwrap_or_else(|e: Error| {
                            eprintln!("connection died, error was: {:?}", e)
                        })
                    }
                })
            })
            .boxed())
        .boxed();

    warp::serve(routes).run(([0, 0, 0, 0], port)).await
}
