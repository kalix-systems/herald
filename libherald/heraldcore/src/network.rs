use crate::{config::Config, errors::HErr, message as db};
use chrono::prelude::*;
use herald_common::{
    read_cbor, send_cbor, ClientMessageAck, GlobalId, MessageStatus, MessageToClient,
    MessageToPeer, MessageToServer, RawMsg, UserId,
};
use lazy_static::*;
use serde::Serialize;
use std::{
    env,
    net::{SocketAddr, SocketAddrV4},
};
use tokio::{net::*, prelude::*};

const DEFAULT_PORT: u16 = 8000;
const DEFAULT_SERVER_IP_ADDR: [u8; 4] = [127, 0, 0, 1];

lazy_static! {
    static ref SERVER_ADDR: SocketAddr = match env::var("SERVER_ADDR") {
        Ok(addr) => addr.parse().unwrap_or_else(|e| {
            eprintln!("Provided address {} is invalid: {}", addr, e);
            std::process::abort();
        }),
        Err(_) => SocketAddr::V4(SocketAddrV4::new(
            DEFAULT_SERVER_IP_ADDR.into(),
            DEFAULT_PORT
        )),
    };
}

/// Initializes connection with the server.
pub async fn open_connection() -> Result<TcpStream, HErr> {
    println!("Client connecting to {}", *SERVER_ADDR);
    let mut stream = TcpStream::connect(*SERVER_ADDR).await?;
    login(&mut stream).await?;
    Ok(stream)
}

/// Login
pub async fn login<S: AsyncWrite + Unpin>(stream: &mut S) -> Result<(), HErr> {
    let gid = GlobalId {
        did: 0,
        uid: Config::static_id()?,
    };
    send_cbor(stream, &gid).await?;
    Ok(())
}

fn form_ack(update_code: MessageStatus, message_id: i64) -> MessageToPeer {
    let ack = ClientMessageAck {
        update_code,
        message_id,
    };
    MessageToPeer::Ack(ack)
}

// note: this should never fail, but I'm returning a result until I read `serde_cbor` more closely
fn form_push(to: UserId, msg: MessageToPeer) -> Result<MessageToServer, HErr> {
    Ok(MessageToServer::SendMsg {
        to,
        body: serde_cbor::to_vec(&msg)?.into(),
    })
}

// TODO: consider making this async?
// sqlite is fast so might not be worth it on an ssd
fn handle_msg(from: UserId, body: String, time: DateTime<Utc>) -> Result<MessageToServer, HErr> {
    let recipient = Config::static_id()?;
    let (row, _) = db::Messages::add_message(
        &from,
        &recipient,
        &body,
        Some(time),
        MessageStatus::Inbound, //all messages are inbound
    )?;
    form_push(from, form_ack(MessageStatus::RecipReceivedAck, row))
}

fn handle_ack(from: UserId, ack: ClientMessageAck) -> Result<(), HErr> {
    let ClientMessageAck {
        message_id,
        update_code,
    } = ack;
    db::Messages::update_status(from.as_str(), message_id, update_code)?;
    Ok(())
}

fn handle_push(
    from: GlobalId,
    body: MessageToPeer,
    time: DateTime<Utc>,
) -> Result<Option<MessageToServer>, HErr> {
    use MessageToPeer::*;
    match body {
        Message(s) => handle_msg(from.uid, s, time).map(Some),
        Ack(a) => handle_ack(from.uid, a).map(|_| None),
    }
}

// TODO: make this handle request responses - that'll end up making this more object-y
pub fn handle_server_msg(msg: MessageToClient) -> Result<Option<MessageToServer>, HErr> {
    use MessageToClient::*;
    match msg {
        Push { from, body, time } => {
            let msg = serde_cbor::from_slice(&body)?;
            handle_push(from, msg, time)
        }
        _ => unimplemented!(),
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::db::DBTable;
//     use herald_common::*;
//     use serial_test_derive::serial;
//     use std::process;

//     fn boot_server() -> std::process::Child {
//         process::Command::new("cargo")
//             .args(&["build", "--bin", "stupid"])
//             .current_dir("../../server")
//             .output()
//             .expect("Failed to start server");

//         let child = process::Command::new("cargo")
//             .args(&["run", "--bin", "stupid"])
//             .current_dir("../../server")
//             .spawn()
//             .expect("Failed to start server");

//         std::thread::sleep(std::time::Duration::from_secs(1));

//         child
//     }

//     #[test]
//     fn serialize() {
//         let msg = form_text_message(String::from("hello"), String::from("world"))
//             .expect("can't build message");
//         let ack = form_server_ack(super::UserId::from("Person"), MessageStatus::NoAck, 0)
//             .expect("can't build ack");

//         match ack {
//             MessageToServer::SendMsg { body, .. } => match serde_cbor::de::from_slice(&body) {
//                 Ok(Body::Ack(ClientMessageAck {
//                     update_code,
//                     message_id,
//                 })) => assert_eq!((update_code, message_id), (MessageStatus::NoAck, 0)),
//                 _ => panic!("Ack was serialized to wrong type"),
//             },
//             _ => panic!("ack was mistyped"),
//         }

//         match msg {
//             MessageToServer::SendMsg { body, .. } => match serde_cbor::de::from_slice(&body) {
//                 Ok(Body::Message(string)) => assert_eq!(String::from("world"), string),
//                 _ => panic!("msg was serialized to wrong type"),
//             },
//             _ => panic!("msg was mistyped"),
//         }
//     }

//     #[test]
//     #[serial]
//     fn register_self() {
//         crate::config::Config::drop_table().unwrap();
//         crate::config::Config::create_table().unwrap();
//         crate::config::Config::new("hello".to_string(), None, None, None, None)
//             .expect("could not create config");

//         let mut child = boot_server();

//         let mut stream = super::open_connection().unwrap_or_else(|e| {
//             eprintln!("connection failed, {}", e);
//             child.kill().expect("Failed to kill child");
//             panic!()
//         });

//         super::send_text_message(super::UserId::from("hello"), "world".into(), &mut stream)
//             .unwrap_or_else(|e| {
//                 eprintln!("msg to self failed, {}", e);
//                 child.kill().expect("Failed to kill child");
//                 panic!()
//             });

//         super::read_from_server(&mut stream).unwrap_or_else(|e| {
//             eprintln!("failed to receive from server, {}", e);
//             child.kill().expect("Failed to kill child");
//             panic!()
//         });

//         super::read_from_server(&mut stream).unwrap_or_else(|e| {
//             eprintln!("failed to receive ack from server, {}", e);
//             child.kill().expect("Failed to kill child");
//             panic!()
//         });

//         child.kill().expect("Failed to kill child");
//     }

//     // #[test]
//     // #[serial]
//     // fn message_to_self() {
//     // crate::contact::Contacts::drop_table().unwrap();
//     // crate::contact::Contacts::create_table().unwrap();
//     // crate::message::Messages::drop_table().unwrap();
//     // crate::message::Messages::create_table().unwrap();
//     // crate::config::Config::drop_table().unwrap();
//     // crate::config::Config::create_table().unwrap();
//     // crate::config::Config::new("hello".to_string(), None, None, None, None)
//     //     .expect("could not create config");
//     //     process::Command::new("cargo")
//     //         .args(&["build", "--bin", "stupid"])
//     //         .current_dir("../../server")
//     //         .output()
//     //         .expect("Failed to start server");

//     //     let mut child = process::Command::new("cargo")
//     //         .args(&["run", "--bin", "stupid"])
//     //         .current_dir("../../server")
//     //         .spawn()
//     //         .expect("Failed to start server");

//     //     println!("sleeping thread");
//     //     std::thread::sleep(std::time::Duration::from_secs(1));

//     //     let mut stream = match super::open_connection() {
//     //         Ok(stream) => stream,
//     //         Err(e) => {
//     //             child.kill().expect("Failed to kill child");
//     //             panic!("connection could not be opened: {}", e);
//     //         }
//     //     };
//     //     println!("Registering user");
//     //     match super::register(super::UserId::from("hello"), &mut stream) {
//     //         Ok(_) => {}
//     //         Err(e) => {
//     //             child.kill().expect("Failed to kill child");
//     //             panic!("registration failed, {}", e);
//     //         }
//     //     }
//     //     println!("Forming message");
//     //     if let Ok(super::MessageToServer::SendMsg { to, body }) =
//     //         super::form_text_message("hello".to_string(), "message content".to_string())
//     //     {
//     //         super::send_text_message(to, body, &mut stream).expect("could not send message");
//     //         std::thread::sleep(std::time::Duration::from_secs(3));
//     //         match super::read_from_server(&mut stream) {
//     //             Ok(()) => println!(" read from server okay!"),
//     //             _ => panic!("failed to read message from server!"),
//     //         }
//     //         child.kill().expect("Failed to kill child");
//     //         let messages = crate::message::Messages::get_conversation(&"hello")
//     //             .expect("could not get messages from DB");

//     //         assert_eq!(&messages[0].body, &"message content");
//     //         println!("fetched message successfully!");
//     //     } else {
//     //         child.kill().expect("Failed to kill child");
//     //         panic!("Could not conver form text message");
//     //     }
//     // }
// }
