use std::env;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};

type Error = Box<dyn std::error::Error>;

const INADDR_ANY: [u8; 4] = [0, 0, 0, 0];
const LOCALHOST: [u8; 4] = [127, 0, 0, 1];

const PORT: u16 = 8000;
const BUFSIZE: usize = 4096;

fn handle_client(mut stream: TcpStream, buf: &mut [u8]) -> Result<(), Error> {
    let len = stream.read(buf)?;
    let message = str::from_utf8(&buf[..len])?;

    print!("{message}");

    Ok(())
}

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().skip(1).collect();

    match args[0].as_str() {
        "server" => server()?,
        "client" => client()?,
        _ => panic!("not found"),
    };

    Ok(())
}

fn server() -> Result<(), Error> {
    let addr = SocketAddr::from((INADDR_ANY, PORT));

    let listener = TcpListener::bind(addr)?;
    let mut buf = [0; BUFSIZE];

    if let Some(stream) = listener.incoming().next() {
        handle_client(stream?, &mut buf)?;
    }

    Ok(())
}

fn client() -> Result<(), Error> {
    let buf = "Hello, Socket Programming\n";

    let addr = SocketAddr::from((LOCALHOST, PORT));
    let mut stream = TcpStream::connect(addr)?;

    stream.write_all(buf.as_bytes())?;

    Ok(())
}
