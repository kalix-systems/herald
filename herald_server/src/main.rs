#[macro_use]
extern crate lazy_static;

use futures::future;
use herald_common::*;
use herald_server::protocol::*;
use server_errors::Error;
use std::net::IpAddr;
use tarpc::server::{Channel, Handler};
use tokio::prelude::*;

lazy_static! {
    static ref HANDLER: State = State::new();
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let addr = IpAddr::from([0, 0, 0, 0]);

    let rpc_fut = transport::listen(&(addr, RPC_PORT).into())?
        .filter_map(|r| future::ready(r.ok()))
        .map(tarpc::server::BaseChannel::with_defaults)
        .max_concurrent_requests_per_channel(10)
        .for_each(|channel| {
            tokio::spawn(channel.respond_with(HANDLER.serve()).execute());
            future::ready(())
        });

    let login_fut = tokio::net::tcp::TcpListener::bind((addr, TCP_PORT))
        .await?
        .incoming()
        .filter_map(|r| future::ready(r.ok()))
        .for_each(|s| {
            tokio::spawn(async move {
                match HANDLER.handle_login(s).await {
                    Ok(()) => {}
                    Err(e) => eprintln!("connection closed with error {:?}", e),
                }
            });
            future::ready(())
        });

    futures::join!(rpc_fut, login_fut);

    Ok(())
}
