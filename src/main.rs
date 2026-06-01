use std::env;

use socket_programming::tcp;

type Error = Box<dyn std::error::Error>;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().skip(1).collect();

    match args[0].as_str() {
        "server" => tcp::server::run()?,
        "client" => tcp::client::run()?,
        _ => panic!("not found"),
    };

    Ok(())
}
