use super::*;
use warp::{filters::ws, Filter};

pub async fn serve(state: &'static State, port: u16) {
    use warp::filters::method;
    let route_get = {
        use get::*;

        warp::path("echo")
            .and(warp::filters::body::concat())
            .map(|b: warp::body::FullBody| b.bytes().to_vec())
            .or(mk_filter!(state, keys_of))
            .or(mk_filter!(state, key_info))
            .or(mk_filter!(state, keys_exist))
            .or(mk_filter!(state, users_exist))
    };
    let route_post = {
        use post::*;
        mk_filter!(state, register)
            .or(mk_filter!(state, new_key))
            .or(mk_filter!(state, dep_key))
            .or(push_filter!(state, push_users))
            .or(push_filter!(state, push_devices))
    };

    let routes = method::get2()
        .and(route_get)
        .or(method::post2().and(route_post))
        .or(warp::path("login").and(ws::ws2()).map(move |w: ws::Ws2| {
            w.on_upgrade(move |w: ws::WebSocket| {
                async move {
                    state
                        .handle_login(w)
                        .await
                        .unwrap_or_else(|e: Error| eprintln!("connection died, error was: {:?}", e))
                }
            })
        }));

    warp::serve(routes).run(([0, 0, 0, 0], port)).await
}
