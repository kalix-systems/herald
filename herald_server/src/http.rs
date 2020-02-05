use super::*;
use futures::{future::*, sink::*, stream::*};
use warp::{filters::ws, Filter};

#[derive(Debug)]
struct Rejection(anyhow::Error);

impl warp::reject::Reject for Rejection {}

macro_rules! mk_filter {
    ($this: expr, $f: ident) => {
        warp::path(stringify!($f))
            .boxed()
            .and(::warp::filters::body::bytes())
            .boxed()
            .and_then(move |b: Bytes| async move {
                let r1: Result<Vec<u8>, anyhow::Error> = req_handler_async($this, b, State::$f).await;
                let r2: Result<Vec<u8>, ::warp::reject::Rejection> =
                    r1.map_err(|e| ::warp::reject::custom(Rejection(e)));
                r2
            })
            .boxed()
    };
    ($this:expr,$f:ident,) => {mk_filter!($this,$f)};
    ($this: expr, $f: ident, $($fs: ident),+) => {
        mk_filter!($this,$f).or(mk_filter!($this, $($fs),+).boxed())
    };
    ($this: expr, $f: ident, $($fs: ident),+,) => {
        mk_filter!($this, $f, $($fs),+)
    };
}

pub async fn serve(
    state: &'static State,
    port: u16,
) {
    let routes = {
        mk_filter!(
            state,
            get_sigchain,
            recip_exists,
            new_sig,
            new_prekeys,
            get_prekeys,
            push,
            register,
        )
        .or(warp::path("login")
            .boxed()
            .and(ws::ws().boxed())
            .boxed()
            .map(move |w: ws::Ws| {
                w.on_upgrade(move |w: ws::WebSocket| {
                    async move {
                        let (wtx, wrx) = w.split();
                        let mut tx = wtx.with(|b: Bytes| {
                            async move {
                                    Ok::<_, server_errors::Error>(ws::Message::binary(b.to_vec()))
                                }
                                .boxed()
                        });
                        let mut rx = wrx
                            .filter_map(|m: Result<ws::Message, warp::Error>| {
                                async move {
                                    m.map(|m| {
                                        if m.is_binary() {
                                            Some(m.into_bytes())
                                        } else {
                                            None
                                        }
                                    })
                                    .transpose()
                                }
                                .boxed()
                            })
                            .boxed();
                        state
                            .handle_auth_ws(&mut tx, &mut rx)
                            .boxed()
                            .await
                            .unwrap_or_else(|e| eprintln!("connection died, error was: {:?}", e))
                    }
                    .boxed()
                })
            })
            .boxed())
        .boxed()
    };

    warp::serve(routes).run(([0u8, 0, 0, 0], port)).await
}
