use std::io::{Read, Write, stdin, stdout};
use std::net::{SocketAddr, TcpStream};

use crate::Error;
use crate::{BUFSIZE, LOCALHOST, PORT};

pub fn run() -> Result<(), Error> {
    let mut send_message = String::new();
    let mut buf = [0; BUFSIZE];

    let addr = SocketAddr::from((LOCALHOST, PORT));
    let mut stream = TcpStream::connect(addr)?;

    loop {
        send_message.clear();

        print!("input message: ");
        stdout().flush()?;
        let read_byte = stdin().read_line(&mut send_message)?;

        if read_byte == 0 {
            return Ok(());
        }

        stream.write_all(send_message.as_bytes())?;
        let len = stream.read(&mut buf)?;

        if len == 0 {
            return Ok(());
        }

        print!("{}", str::from_utf8(&buf[..len])?);
    }
}
