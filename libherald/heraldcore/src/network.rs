use std::net::{Ipv4Addr, SocketAddrV4, TcpStream};

fn connect() {
    let socket = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 8000);

    let steam = TcpStream::connect(socket);

    // send length
    // send globalid
    // read length
    // read message
}

#[cfg(test)]
mod tests {
    use super::connect;
    use std::{process::Command, thread};
}
