use crate::errors::HErr;
use herald_common::{GlobalId, MessageToClient, MessageToServer, RawMsg, UserId};
use serde::{Deserialize, Serialize};
use std::{
    io::{Read, Write},
    net::{SocketAddrV4, TcpStream},
};

const PORT: u16 = 8000;
const SERVER_ADDR: [u8; 4] = [127, 0, 0, 1];

/// Sends `data` such as messages, Registration requests,
/// and metadata to the server.
pub fn send_to_server<T: Serialize>(data: &T, stream: &mut TcpStream) -> Result<(), HErr> {
    let msg_v = serde_cbor::to_vec(data)?;
    stream.write_all(&msg_v.len().to_le_bytes())?;
    stream.write_all(msg_v.as_slice())?;
    Ok(())
}

/// Reads inbound data from the server
/// along a tcp stream.
pub fn read_from_server<T: for<'de> Deserialize<'de>>(stream: &mut TcpStream) -> Result<T, HErr> {
    let mut buf = [0u8; 8];
    stream.read_exact(&mut buf)?;
    let len = u64::from_le_bytes(buf) as usize;
    let mut buf = vec![0u8; len];
    stream.read_exact(&mut buf)?;
    Ok(serde_cbor::from_slice(&buf)?)
}

/// Registers `user_id` on the server.
pub fn register(user_id: UserId) -> Result<(), HErr> {
    let socket = SocketAddrV4::new(SERVER_ADDR.into(), PORT);

    let gid = GlobalId {
        did: 0,
        uid: user_id,
    };

    let msg = MessageToServer::SendMsg {
        to: user_id,
        text: RawMsg::from(""),
    };

    let mut stream = TcpStream::connect(socket)?;
    send_to_server(&gid, &mut stream)?;
    send_to_server(&msg, &mut stream)?;

    // Shim code to ack self because the server is ~stupid~
    read_from_server(&mut stream)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::process;

    #[test]
    fn register() {
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

        if super::register(super::UserId::from("hello").unwrap()).is_err() {
            child.kill().expect("Failed to kill child");
        }

        child.kill().expect("Failed to kill child");
    }
}
