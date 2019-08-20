use crate::errors::HErr;
use herald_common::{GlobalId, MessageToServer, RawMsg, UserId};
use serde_cbor::to_vec;
use std::{
    io::Write,
    net::{Ipv4Addr, SocketAddrV4, TcpStream},
};

pub fn send_message() -> Result<(), HErr> {
    let socket = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 8000);

    let gid = GlobalId {
        did: 0,
        uid: UserId::from("me")?,
    };

    let msg = MessageToServer::SendMsg {
        to: UserId::from("userid")?,
        text: RawMsg::from("Hello World"),
    };

    let mut stream = TcpStream::connect(socket).unwrap();

    let gid_v = serde_cbor::to_vec(&gid).unwrap();
    stream.write_all(&gid_v.len().to_le_bytes()).unwrap();
    stream.write_all(gid_v.as_slice()).unwrap();

    let msg_v = serde_cbor::to_vec(&msg).unwrap();
    stream.write_all(&msg_v.len().to_le_bytes()).unwrap();
    stream.write_all(msg_v.as_slice()).unwrap();
    // send length
    // send globalid
    // read length
    // read message
    Ok(())
}
