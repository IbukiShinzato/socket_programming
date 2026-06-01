use std::net::{SocketAddr, UdpSocket};

use crate::Error;
use crate::{LOCALHOST, PORT};

pub fn run() -> Result<(), Error> {
    let buf = "Hello, Socket Programming\n";

    let addr = SocketAddr::from((LOCALHOST, 0));
    let socket = UdpSocket::bind(addr)?;

    let src = SocketAddr::from((LOCALHOST, PORT));

    socket.send_to(buf.as_bytes(), src)?;

    Ok(())
}
