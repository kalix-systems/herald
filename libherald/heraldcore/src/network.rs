use crate::errors::HErr;
use herald_common::{GlobalId, MessageToServer, RawMsg, UserId};
use std::{
    io::{Read, Write},
    net::{SocketAddrV4, TcpStream},
};

const PORT: u16 = 8000;
const SERVER_ADDR: [u8; 4] = [127, 0, 0, 1];

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

    let mut stream = TcpStream::connect(socket).unwrap();

    let gid_v = serde_cbor::to_vec(&gid).unwrap();
    stream.write_all(&gid_v.len().to_le_bytes()).unwrap();
    stream.write_all(gid_v.as_slice()).unwrap();

    let msg_v = serde_cbor::to_vec(&msg).unwrap();
    stream.write_all(&msg_v.len().to_le_bytes()).unwrap();
    stream.write_all(msg_v.as_slice()).unwrap();

    let mut buf = [0u8; 8];
    stream.read_exact(&mut buf)?;
    let len = u64::from_le_bytes(buf) as usize;
    let mut buf = vec![0u8; len];
    stream.read_exact(&mut buf)?;
    serde_cbor::from_slice(&buf)?;
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
