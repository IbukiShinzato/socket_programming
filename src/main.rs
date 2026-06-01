use std::{env, io};

use socket_programming::{Error, tcp, udp};

fn invalid_args_error() -> Error {
    Box::new(io::Error::new(
        io::ErrorKind::InvalidInput,
        "invalid protocol or mode",
    ))
}

fn validate_args(args: &[String]) -> Result<(), Error> {
    if args.len() != 2 {
        eprintln!("Usage: cargo run -- <tcp or udp> <server or client>");
        return Err(invalid_args_error());
    }

    Ok(())
}

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().skip(1).collect();
    validate_args(&args)?;

    let (protocol, mode) = (&args[0], &args[1]);

    match (protocol.as_str(), mode.as_str()) {
        ("tcp", "server") => tcp::server::run(),
        ("tcp", "client") => tcp::client::run(),
        ("udp", "server") => udp::server::run(),
        ("udp", "client") => {
            return Err(invalid_args_error());
        }
        _ => {
            return Err(invalid_args_error());
        }
    }?;

    Ok(())
}
