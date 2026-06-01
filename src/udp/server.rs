use std::net::{SocketAddr, UdpSocket};

use crate::Error;
use crate::{BUFSIZE, INADDR_ANY, PORT};

pub fn run() -> Result<(), Error> {
    let addr = SocketAddr::from((INADDR_ANY, PORT));

    let socket = UdpSocket::bind(addr)?;
    let mut buf = [0; BUFSIZE];

    let (amt, _src) = socket.recv_from(&mut buf)?;
    let message = str::from_utf8(&buf[..amt])?;

    print!("{message}");

    Ok(())
}
