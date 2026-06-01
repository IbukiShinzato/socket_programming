use std::io::Read;
use std::net::{SocketAddr, TcpListener, TcpStream};

use super::{BUFSIZE, INADDR_ANY, PORT};
use crate::Error;

fn handle_client(mut stream: TcpStream, buf: &mut [u8]) -> Result<(), Error> {
    let len = stream.read(buf)?;
    let message = str::from_utf8(&buf[..len])?;

    print!("{message}");

    Ok(())
}

pub fn run() -> Result<(), Error> {
    let addr = SocketAddr::from((INADDR_ANY, PORT));

    let listener = TcpListener::bind(addr)?;
    let mut buf = [0; BUFSIZE];

    if let Some(stream) = listener.incoming().next() {
        handle_client(stream?, &mut buf)?;
    }

    Ok(())
}
