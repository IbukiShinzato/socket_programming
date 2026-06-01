use std::io::Write;
use std::net::{SocketAddr, TcpStream};

use crate::Error;
use crate::{LOCALHOST, PORT};

pub fn run() -> Result<(), Error> {
    let buf = "Hello, Socket Programming\n";

    let addr = SocketAddr::from((LOCALHOST, PORT));
    let mut stream = TcpStream::connect(addr)?;

    stream.write_all(buf.as_bytes())?;

    Ok(())
}
