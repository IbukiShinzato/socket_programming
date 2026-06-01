use std::env;
use std::io::Read;
use std::net::{SocketAddr, TcpListener, TcpStream};

type Error = Box<dyn std::error::Error>;

const INADDRANY: [u8; 4] = [0, 0, 0, 0];

const PORT: u16 = 8000;
const BUFSIZE: usize = 4096;

fn handle_client(mut stream: TcpStream, buf: &mut [u8]) -> Result<(), Error> {
    let len = stream.read(buf)?;
    let message = str::from_utf8(&buf[..len])?;

    print!("{message}");

    Ok(())
}

fn main() -> Result<(), Error> {
    let _args: Vec<String> = env::args().skip(1).collect();
    let addr = SocketAddr::from((INADDRANY, PORT));

    let listener = TcpListener::bind(addr)?;
    let mut buf = [0; BUFSIZE];

    if let Some(stream) = listener.incoming().next() {
        handle_client(stream?, &mut buf)?;
    }

    Ok(())
}
