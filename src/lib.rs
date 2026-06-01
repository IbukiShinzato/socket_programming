pub mod tcp;
pub mod udp;

pub type Error = Box<dyn std::error::Error>;

const INADDR_ANY: [u8; 4] = [0, 0, 0, 0];
const LOCALHOST: [u8; 4] = [127, 0, 0, 1];

const PORT: u16 = 8000;
const BUFSIZE: usize = 4096;
