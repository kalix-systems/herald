use crate::errors::HErr;
use herald_common::{
    Body, ClientMessageAck, GlobalId, MessageStatus, MessageToClient, MessageToServer, RawMsg,
    UserId,
};
use lazy_static::*;
use serde::Serialize;
use std::{
    env,
    io::{Read, Write},
    net::{SocketAddrV4, TcpStream},
};

const DEFAULT_PORT: u16 = 8000;
const DEFUALT_SERVER_IP_ADDR: [u8; 4] = [127, 0, 0, 1];

lazy_static! {
    static ref SERVER_ADDR: SocketAddrV4 = match env::var("SERVER_ADDR") {
        Ok(addr) => addr.parse().unwrap_or_else(|e| {
            eprintln!("Provided address {} is invalid: {}", addr, e);
            std::process::abort();
        }),
        Err(_) => SocketAddrV4::new(DEFUALT_SERVER_IP_ADDR.into(), DEFAULT_PORT),
    };
}

/// Initializes connection with the server.
pub fn open_connection() -> Result<TcpStream, HErr> {
    let socket: SocketAddrV4 = *SERVER_ADDR;
    println!("Client connecting to {}", *SERVER_ADDR);
    let mut stream = TcpStream::connect(socket)?;
    login(&mut stream)?;
    Ok(stream)
}

/// Sends `data` such as messages, Registration requests,
/// and metadata to the server.
pub fn send_to_server<T: Serialize>(data: &T, stream: &mut TcpStream) -> Result<(), HErr> {
    let msg_v = serde_cbor::to_vec(data)?;
    stream.write_all(&(msg_v.len() as u64).to_le_bytes())?;
    stream.write_all(msg_v.as_slice())?;
    Ok(())
}

/// Reads inbound data from the server
pub fn read_from_server(stream: &mut TcpStream) -> Result<(), HErr> {
    stream.set_nonblocking(true)?;

    let mut buf = [0u8; 8];
    loop {
        match stream.read_exact(&mut buf) {
            Ok(_) => break,
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                //platform specific login needs to be here.
                std::thread::sleep(std::time::Duration::from_secs(2));
            }
            Err(e) => {
                eprintln!("Error reading from server : {}", e);
                return Err(HErr::IoError(e));
            }
        };
    }

    stream.set_nonblocking(false)?;

    let len = u64::from_le_bytes(buf) as usize;
    let mut buf = vec![0u8; len];
    stream.read_exact(&mut buf)?;

    let msg = serde_cbor::from_slice(&buf)?;

    match msg {
        MessageToClient::Push { from, body, time } => {
            match serde_cbor::de::from_slice(&body) {
                Ok(Body::Message(body)) => {
                    println!("Message Received. Decoding.");
                    let recipient = crate::config::Config::static_id()?;
                    let GlobalId { uid: from, .. } = from;
                    let (row, _) = crate::message::Messages::add_message(
                        from.to_string().as_str(),
                        recipient.as_str(),
                        body.as_str(),
                        Some(time),
                        MessageStatus::Inbound, //all messages are inbound
                    )?;
                    send_ack(from, MessageStatus::RecipReceivedAck, row, stream)?;
                }
                Ok(Body::Ack(ClientMessageAck {
                    update_code,
                    message_id,
                })) => {
                    println!("ACK Received. Decoding.");
                    crate::message::Messages::update_status(
                        from.uid.to_string().as_str(),
                        message_id,
                        update_code,
                    )?
                }
                Err(e) => {
                    eprintln!("Error decoding new message {}", e);
                }
            };
        }
        _ => unimplemented!(),
    }
    Ok(())
}

/// Registers `user_id` on the server.
pub fn register(user_id: UserId, stream: &mut TcpStream) -> Result<(), HErr> {
    let gid = GlobalId {
        did: 0,
        uid: user_id.clone(),
    };

    let msg = MessageToServer::SendMsg {
        to: user_id.clone(),
        body: serde_cbor::ser::to_vec(&Body::Message(String::from("").into()))?.into(),
    };

    send_to_server(&gid, stream)?;
    send_to_server(&msg, stream)?;

    // Shim code to ack self because the server is ~stupid~
    read_from_server(stream)?;

    Ok(())
}

/// Login
pub fn login(stream: &mut TcpStream) -> Result<(), HErr> {
    let gid = GlobalId {
        did: 0,
        uid: crate::config::Config::static_id()?,
    };

    send_to_server(&gid, stream)
}

/// serializes and forms text messages to be send to the server
pub fn form_text_message(to: String, body: String) -> Result<MessageToServer, HErr> {
    let to = UserId::from(&to);
    Ok(MessageToServer::SendMsg {
        to,
        body: serde_cbor::ser::to_vec(&Body::Message(body))?.into(),
    })
}

/// Sends message to server.
pub fn send_text_message(to: UserId, body: RawMsg, stream: &mut TcpStream) -> Result<(), HErr> {
    let msg = MessageToServer::SendMsg { to, body };
    send_to_server(&msg, stream)?;
    Ok(())
}

/// serializes forms a server ack
pub fn form_server_ack(
    to: UserId,
    update_code: MessageStatus,
    message_id: i64,
) -> Result<MessageToServer, HErr> {
    let ack = ClientMessageAck {
        update_code,
        message_id,
    };
    Ok(MessageToServer::SendMsg {
        to,
        body: serde_cbor::ser::to_vec(&Body::Ack(ack))?.into(),
    })
}

/// Sends Ack to server.
pub fn send_ack(
    to: UserId,
    update_code: MessageStatus,
    message_id: i64,
    stream: &mut TcpStream,
) -> Result<(), HErr> {
    send_to_server(&form_server_ack(to, update_code, message_id)?, stream)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::db::DBTable;
    use serial_test_derive::serial;
    use std::process;

    #[test]
    #[serial]
    fn register() {
        crate::contact::Contacts::drop_table().unwrap();
        crate::contact::Contacts::create_table().unwrap();
        crate::message::Messages::drop_table().unwrap();
        crate::message::Messages::create_table().unwrap();
        crate::config::Config::drop_table().unwrap();
        crate::config::Config::create_table().unwrap();
        crate::config::Config::new("hello".to_string(), None, None, None, None)
            .expect("could not create config");

        process::Command::new("cargo")
            .args(&["build", "--bin", "stupid"])
            .current_dir("../../server")
            .output()
            .expect("Failed to start server");

        let mut child = process::Command::new("cargo")
            .args(&["run", "--bin", "stupid"])
            .current_dir("../../server")
            .spawn()
            .expect("Failed to start server");

        std::thread::sleep(std::time::Duration::from_secs(1));

        let mut stream = match super::open_connection() {
            Ok(stream) => stream,
            Err(e) => {
                child.kill().expect("Failed to kill child");
                panic!("connection failed, {}", e);
            }
        };

        std::thread::sleep(std::time::Duration::from_secs(2));

        match super::register(super::UserId::from("hello"), &mut stream) {
            Err(e) => {
                child.kill().expect("Failed to kill child");
                panic!("Registration failed , {}", e);
            }
            _ => {}
        };
        child.kill().expect("Failed to kill child");
    }

    // #[test]
    // #[serial]
    // fn message_to_self() {
    //     process::Command::new("cargo")
    //         .args(&["build", "--bin", "stupid"])
    //         .current_dir("../../server")
    //         .output()
    //         .expect("Failed to start server");

    //     let mut child = process::Command::new("cargo")
    //         .args(&["run", "--bin", "stupid"])
    //         .current_dir("../../server")
    //         .spawn()
    //         .expect("Failed to start server");

    //     println!("sleeping thread");
    //     std::thread::sleep(std::time::Duration::from_secs(1));

    //     let mut stream = match super::open_connection() {
    //         Ok(stream) => stream,
    //         Err(e) => {
    //             child.kill().expect("Failed to kill child");
    //             panic!("connection could not be opened: {}", e);
    //         }
    //     };
    //     println!("Registering user");
    //     match super::register(super::UserId::from("hello"), &mut stream) {
    //         Ok(_) => {}
    //         Err(e) => {
    //             child.kill().expect("Failed to kill child");
    //             panic!("registration failed, {}", e);
    //         }
    //     }
    //     println!("Forming message");
    //     if let Ok(super::MessageToServer::SendMsg { to, body }) =
    //         super::form_text_message("hello".to_string(), "message content".to_string())
    //     {
    //         super::send_text_message(to, body, &mut stream).expect("could not send message");
    //         std::thread::sleep(std::time::Duration::from_secs(3));
    //         match super::read_from_server(&mut stream) {
    //             Ok(()) => println!(" read from server okay!"),
    //             _ => panic!("failed to read message from server!"),
    //         }
    //         child.kill().expect("Failed to kill child");
    //         let messages = crate::message::Messages::get_conversation(&"hello")
    //             .expect("could not get messages from DB");

    //         assert_eq!(&messages[0].body, &"message content");
    //         println!("fetched message successfully!");
    //     } else {
    //         child.kill().expect("Failed to kill child");
    //         panic!("Could not conver form text message");
    //     }
    // }
}
