use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::os::fd::{AsFd, AsRawFd, RawFd};
use std::thread;

use nix::sys::select::{FdSet, select};

use crate::Error;
use crate::{BUFSIZE, INADDR_ANY, PORT};

fn handle_client(stream: &mut TcpStream, buf: &mut [u8]) -> Result<bool, Error> {
    let len = stream.read(buf)?;
    if len == 0 {
        return Ok(false);
    }

    stream.write_all(&buf[..len])?;
    print!("recv: {}", str::from_utf8(&buf[..len])?);

    Ok(true)
}

pub fn select_server() -> Result<(), Error> {
    let addr = SocketAddr::from((INADDR_ANY, PORT));
    let listener = TcpListener::bind(addr)?;

    let listener_fd = listener.as_raw_fd();
    let mut tcp_map: HashMap<RawFd, TcpStream> = HashMap::new();
    let mut buf = [0; BUFSIZE];

    loop {
        let max_fd = tcp_map
            .keys()
            .copied()
            .max()
            .map_or(listener_fd, |fd| fd.max(listener_fd));

        let (listener_ready, ready_client_fds) = {
            let mut fds = FdSet::new();

            fds.insert(listener.as_fd());

            for stream in tcp_map.values() {
                fds.insert(stream.as_fd());
            }

            select(Some(max_fd + 1), Some(&mut fds), None, None, None)?;

            let listener_ready = fds.contains(listener.as_fd());

            let ready_client_fds: Vec<RawFd> = tcp_map
                .iter()
                .filter_map(|(&fd, stream)| {
                    if fds.contains(stream.as_fd()) {
                        Some(fd)
                    } else {
                        None
                    }
                })
                .collect();

            (listener_ready, ready_client_fds)
        };

        if listener_ready {
            let (stream, _addr) = listener.accept()?;
            tcp_map.insert(stream.as_raw_fd(), stream);
        }

        let mut closed_fds = vec![];

        for fd in ready_client_fds {
            if let Some(stream) = tcp_map.get_mut(&fd) {
                let connected = handle_client(stream, &mut buf)?;

                if !connected {
                    closed_fds.push(fd);
                }
            }
        }

        for fd in closed_fds {
            tcp_map.remove(&fd);
        }
    }
}

pub fn thread_server() -> Result<(), Error> {
    let addr = SocketAddr::from((INADDR_ANY, PORT));
    let listener = TcpListener::bind(addr)?;

    loop {
        let (mut stream, _addr) = listener.accept()?;
        thread::spawn(move || {
            let mut buf = [0; BUFSIZE];
            loop {
                match handle_client(&mut stream, &mut buf) {
                    Ok(true) => continue,
                    Ok(false) => break,
                    Err(e) => {
                        eprintln!("{e}");
                        break;
                    }
                }
            }
        });
    }
}
