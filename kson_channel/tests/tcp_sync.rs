use kson::prelude::*;
use kson_channel::*;
use std::net::{TcpListener, TcpStream};
use std::thread::spawn;

const NUM_STREAMS: usize = 100;
const NUM_ITERS: usize = 10;

const PACKET_SIZE: usize = 1024;

const MSG: &'static [u8] = &[1; PACKET_SIZE * 10];

#[test]
fn echo() {
    let addr: (&str, u16) = (
        "127.0.0.1",
        port_check::free_local_port().expect("failed to find free port"),
    );

    let server = spawn(move || {
        let mut joins = Vec::with_capacity(NUM_STREAMS);

        let listener = TcpListener::bind(addr.clone()).expect("failed to bind to free port");

        for _ in 0..NUM_STREAMS {
            let (stream, _) = listener.accept().expect("failed to get next tcp stream");

            joins.push(spawn(move || {
                let mut framed = Framed::new(stream);

                for i in 0..NUM_ITERS {
                    let msg: Bytes = framed
                        .read_de_sync()
                        .expect(&format!("Failed to read ping {}", i));
                    assert_eq!(msg, Bytes::from_static(MSG));
                    framed
                        .write_ser_sync(&Bytes::from_static(MSG))
                        .expect(&format!("Failed to write pong {}", i));
                }
            }));
        }

        for f in joins {
            f.join().expect("a server task panicked");
        }
    });

    let clients = spawn(move || {
        let mut joins = Vec::with_capacity(NUM_STREAMS);

        for _ in 0..NUM_STREAMS {
            joins.push(spawn(move || {
                let stream = TcpStream::connect(addr.clone()).expect("failed to connect to server");

                let mut framed = Framed::new(stream);
                for i in 0..NUM_ITERS {
                    framed
                        .write_ser_sync(&Bytes::from_static(MSG))
                        .expect(&format!("Failed to write ping {}", i));
                    let msg: Bytes = framed
                        .read_de_sync()
                        .expect(&format!("Failed to write pong {}", i));
                    assert_eq!(msg, Bytes::from_static(MSG));
                }
            }));
        }

        for f in joins {
            f.join().expect("a server task panicked");
        }
    });

    server.join().expect("server panicked");
    clients.join().expect("client panicked");
}
