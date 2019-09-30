#[macro_use]
extern crate lazy_static;

use herald_server::protocol::*;
use std::process::Command;

lazy_static! {
    static ref HANDLER: State = State::new();
}

// const PGVOLUME: &'static str = "/tmp/postgres_volume:/var/lib/postgresql/data";

#[warp::tokio_rt]
async fn main() {
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
        .args(&["start", "pg-docker"])
        .output()
        .expect("failed to start docker container");

    // println!("fetching docker image");
    // // Command::new("docker")
    //     .args(&["pull", "postgres"])
    //     .output()
    //     .expect("failed to pull postgres image");
    // Command::new("docker")
    //     .arg("run")
    //     .arg("--rm")
    //     .args(&["--name", "pg-docker"])
    //     .args(&["-e", "POSTGRESS_PASSWORD=docker"])
    //     .args(&["-d", "-p", "5432:5432"])
    //     .args(&["-v", PGVOLUME])
    //     .arg("postgres")
    //     .output()
    //     .expect("failed to start postgres");

    println!("starting server");
    HANDLER.serve(8080).await;

    println!("server stopped - shutting down postgres");
    Command::new("docker")
        .args(&["kill", "pg-docker"])
        .output()
        .expect("failed to kill postgres");

    println!("all done");
}
