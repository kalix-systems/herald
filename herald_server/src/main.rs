use futures::{future, stream::StreamExt};
use herald_server::*;
use once_cell::sync::Lazy;
use rustop::opts;
use server_protocol::State;
use std::{fs, net::SocketAddr, ops::Deref, path::Path};

static STATE: Lazy<State> = Lazy::new(|| State::new());

#[tokio::main]
async fn main() {
    let (args, _rest) = opts! {
        synopsis "A server for Herald";
        opt tls_dir:Option<String>, desc:"location for tls certs (defaults to current directory)";
        opt hosts:Vec<String>, desc:"hostnames to generate tls cert for";
        opt port:u16=8080, desc:"port to bind server to";
        opt verbose:bool=false, desc:"verbose error logging";
    }
    .parse_or_exit();

    let dir = Path::new(
        args.tls_dir
            .as_ref()
            .map(String::as_str)
            .unwrap_or("./.tls"),
    );

    let hosts = args.hosts;
    let port = args.port;
    let verbose = args.verbose;

    fs::create_dir_all(dir).expect("failed to create tls directory");

    let fnames = fs::read_dir(dir)
        .expect("failed to check contents of tls directory")
        .collect::<Result<Vec<_>, _>>()
        .expect("failed to get tls dir entries");

    let tkp = if fnames.iter().any(|f| f.file_name() == "cert")
        && fnames.iter().any(|f| f.file_name() == "priv")
    {
        TlsKeyPair::read_from_dir(&dir)
            .expect("failed to read keypair but cert and priv files were present")
    } else {
        let tkp = TlsKeyPair::gen_new_self_signed(hosts)
            .expect("failed to generate tls keypair for hosts");
        tkp.write_to_dir(&dir)
            .expect("failed to write tls key files");
        tkp
    };

    let acceptor = tkp
        .configure_server()
        .expect("failed to create TlsAcceptor from keypair");

    let socket: SocketAddr = ([0u8, 0, 0, 0], port).into();

    let futs = krpc::ws::server::serve(STATE.deref(), socket, acceptor).await;

    futs.for_each(move |f| {
        future::ready({
            tokio::spawn(async move {
                if let Err(e) = f.await {
                    if verbose {
                        eprintln!("{}", e);
                    }
                }
            });
        })
    })
    .await;
}
