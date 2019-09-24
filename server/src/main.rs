#![feature(try_blocks)]

#[macro_use]
extern crate lazy_static;

use herald_server::protocol::*;
use std::process::Command;
use tokio::net::TcpListener;

lazy_static! {
    static ref HANDLER: State = State::new().expect("failed to start server");
}

const PGVOLUME: &'static str = "/tmp/postgres_volume:/var/lib/postgresql/data";

#[tokio::main]
async fn main() {
    println!("fetching docker image");
    Command::new("docker")
        .args(&["pull", "postgres"])
        .output()
        .expect("failed to pull postgres image");

    ctrlc::set_handler(|| {
        println!("");
        println!("quitting");
        Command::new("docker")
            .args(&["kill", "pg-docker"])
            .status()
            .expect("failed to kill postgres");
        std::process::exit(0)
    })
    .expect("failed to setup ctrlc handler");

    println!("starting docker container");
    Command::new("docker")
        .arg("run")
        .arg("--rm")
        .args(&["--name", "pg-docker"])
        .args(&["-e", "POSTGRESS_PASSWORD=docker"])
        .args(&["-d", "-p", "5432:5432"])
        .args(&["-v", PGVOLUME])
        .arg("postgres")
        .output()
        .expect("failed to start postgres");

    println!("starting tcp listener");
    let mut listener = TcpListener::bind("0.0.0.0:8000")
        .await
        .expect("failed to bind tcp listener");

    println!("ready to connect");
    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(async move {
            if let Err(e) = HANDLER.handle_stream(stream).await {
                eprintln!("connection to {} closed - error was {:?}", addr, e);
            }
        });
    }

    Command::new("docker")
        .args(&["kill", "pg-docker"])
        .output()
        .expect("failed to kill postgres");
}
