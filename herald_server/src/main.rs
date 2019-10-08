#[macro_use]
extern crate lazy_static;

use herald_server::protocol::*;
use std::process::Command;

lazy_static! {
    static ref HANDLER: State = State::new();
}

#[tokio::main]
async fn main() {
    HANDLER.serve(8080).await;
}
