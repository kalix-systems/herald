#[macro_use]
extern crate lazy_static;

use server_protocol::State;

lazy_static! {
    static ref HANDLER: State = State::new();
}

#[tokio::main]
async fn main() {
    // herald_server::http::serve(&HANDLER, 8080).await;
}
