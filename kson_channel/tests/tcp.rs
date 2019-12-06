use kson::prelude::*;
use kson_channel::*;
use std::time::Duration;
use tokio::net::*;

const NUM_STREAMS: usize = 100;
const NUM_ITERS: usize = 10;

const TIMEOUT: Duration = Duration::from_secs(1);
const PACKET_SIZE: usize = 1024;

const MSG: &'static [u8] = &[1; PACKET_SIZE * 10];

#[tokio::test]
async fn echo() {
    let addr: (&str, u16) = (
        "127.0.0.1",
        port_check::free_local_port().expect("failed to find free port"),
    );

    let server = tokio::spawn(async move {
        let mut joins = Vec::with_capacity(NUM_STREAMS);

        let mut listener = TcpListener::bind(addr.clone())
            .await
            .expect("failed to bind to free port");

        for _ in 0..NUM_STREAMS {
            let (stream, _) = listener
                .accept()
                .await
                .expect("failed to get next tcp stream");

            joins.push(tokio::spawn(async move {
                let mut framed = Framed::new(stream, TIMEOUT, PACKET_SIZE);

                for i in 0..NUM_ITERS {
                    let msg: Bytes = framed
                        .read_packeted()
                        .await
                        .expect(&format!("Failed to read ping {}", i));
                    assert_eq!(msg, Bytes::from_static(MSG));
                    framed
                        .write_packeted(&Bytes::from_static(MSG))
                        .await
                        .expect(&format!("Failed to write pong {}", i));
                }
            }));
        }

        for f in joins {
            f.await.expect("a server task panicked");
        }
    });

    let clients = tokio::spawn(async move {
        let mut joins = Vec::with_capacity(NUM_STREAMS);

        for _ in 0..NUM_STREAMS {
            joins.push(tokio::spawn(async move {
                let stream = TcpStream::connect(addr.clone())
                    .await
                    .expect("failed to connect to server");

                let mut framed = Framed::new(stream, TIMEOUT, PACKET_SIZE);
                for i in 0..NUM_ITERS {
                    framed
                        .write_packeted(&Bytes::from_static(MSG))
                        .await
                        .expect(&format!("Failed to write ping {}", i));
                    let msg: Bytes = framed
                        .read_packeted()
                        .await
                        .expect(&format!("Failed to write pong {}", i));
                    assert_eq!(msg, Bytes::from_static(MSG));
                }
            }));
        }

        for f in joins {
            f.await.expect("a server task panicked");
        }
    });

    server.await.expect("server panicked");
    clients.await.expect("client panicked");
}
