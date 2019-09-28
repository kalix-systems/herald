use super::*;

impl State {
    pub async fn serve(&'static self, port: u16) {
        use warp::filters::method;
        let route_get = {
            use get::*;
            mk_filter!(self, keys_of)
                .or(mk_filter!(self, key_info))
                .or(mk_filter!(self, keys_exist))
                .or(mk_filter!(self, users_exist))
        };
        let route_post = {
            use post::*;
            mk_filter!(self, register)
                .or(mk_filter!(self, new_key))
                .or(mk_filter!(self, dep_key))
                .or(push_filter!(self, push_users))
                .or(push_filter!(self, push_devices))
        };

        let routes = method::get2()
            .and(route_get)
            .or(method::post2().and(route_post))
            .or(warp::path("login").and(ws::ws2()).map(move |w: ws::Ws2| {
                w.on_upgrade(move |w| {
                    async move {
                        self.handle_login(w)
                            .await
                            .unwrap_or_else(|e| eprintln!("connection died, error was: {:?}", e))
                    }
                })
            }));

        warp::serve(routes).run(([0, 0, 0, 0], port)).await
    }
}
