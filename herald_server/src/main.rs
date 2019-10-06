#[macro_use]
extern crate lazy_static;

use herald_server::protocol::*;
use std::process::Command;

lazy_static! {
    static ref HANDLER: State = State::new();
}

// const PGVOLUME: &'static str = "/tmp/postgres_volume:/var/lib/postgresql/data";

#[tokio::main]
async fn main() {
    ctrlc::set_handler(|| {
        println!("");
        println!("quitting");
        Command::new("docker-compose")
            .arg("down")
            .status()
            .expect("failed to kill docker");
        std::process::exit(0)
    })
    .expect("failed to setup ctrlc handler");

    println!("starting docker container");
    Command::new("docker-compose")
        .args(&["up", "-d"])
        .output()
        .expect("failed to start docker container");

    println!("starting server");
    HANDLER.serve(8080).await;

    println!("server stopped - shutting down postgres");
    Command::new("docker-compose")
        .arg("down")
        .status()
        .expect("failed to kill docker");

    println!("all done");
}
