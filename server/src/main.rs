#![feature(try_blocks)]

#[macro_use]
extern crate lazy_static;

use herald_server::protocol::*;
use std::process::Command;
use tokio::net::TcpListener;

const REDISURL: &'static str = "redis://127.0.0.1/";

lazy_static! {
    static ref HANDLER: State = State::new(REDISURL).expect("failed to connect to redis");
}

#[tokio::main]
async fn main() {
    let mut child = Command::new("redis-server")
        .spawn()
        .expect("failed to start redis");

    let mut listener = TcpListener::bind("0.0.0.0:8000")
        .await
        .expect("failed to bind tcp listener");

    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(async move {
            if let Err(e) = HANDLER.handle_stream(stream).await {
                eprintln!("connection to {} closed - error was {:?}", addr, e);
            }
        });
    }

    child.kill().expect("failed to kill redis");
}
