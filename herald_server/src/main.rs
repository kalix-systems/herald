#[macro_use]
extern crate lazy_static;

use herald_server::protocol::*;
use std::process::Command;

lazy_static! {
    static ref HANDLER: State = State::new();
}

#[tokio::main]
async fn main() {
    println!("starting docker container");
    Command::new("docker-compose")
        .args(&["up", "-d"])
        .output()
        .expect("failed to start docker container");

    println!("starting server");
    HANDLER.serve(8080).await;
}
