use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};

use crate::Error;
use crate::{BUFSIZE, INADDR_ANY, PORT};

fn handle_client(mut stream: TcpStream, buf: &mut [u8]) -> Result<(), Error> {
    loop {
        let len = stream.read(buf)?;
        if len == 0 {
            return Ok(());
        }

        stream.write_all(&buf[..len])?;
        print!("recv: {}", str::from_utf8(&buf[..len])?);
    }
}

pub fn run() -> Result<(), Error> {
    let addr = SocketAddr::from((INADDR_ANY, PORT));

    let listener = TcpListener::bind(addr)?;
    let mut buf = [0; BUFSIZE];

    for stream in listener.incoming() {
        handle_client(stream?, &mut buf)?;
    }

    Ok(())
}
