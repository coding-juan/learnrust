use std::{
    io::{Read, Error as IoErr},
    str::{from_utf8, Utf8Error},
    net::TcpStream
};

type Result<T> = std::result::Result<T, OurError>;

#[derive(Debug)]
enum OurError {
    Io(IoErr),
    Utf8(Utf8Error),
}

impl From<IoErr> for OurError {
    fn from(e: IoErr) -> Self {
        Self::Io(e)
    }
}

impl From<Utf8Error> for OurError {
    fn from(e: Utf8Error) -> Self {
        Self::Utf8(e)
    }
}

fn run() -> Result<()> {
    let mut buf = vec![0u8; 8192];

    let mut stream = TcpStream::connect("127.0.0.1:5000")?;

    loop {
        let bc = stream.read(&mut buf)?;

        if bc == 0 {
            return Ok(());
        }

        let msg = match from_utf8(&buf[..bc]) {
            Ok(m) => m,
            Err(e) => {
                eprintln!("Invalid utf8: {:?}", e);
                continue;
            }
        };
    }
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Fail {:?}", e);
    }
}
