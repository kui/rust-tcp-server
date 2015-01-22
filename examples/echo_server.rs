#![allow(unstable)]

extern crate tcp_server;
#[macro_use] extern crate log;

use std::io::net::tcp::TcpStream;
use std::io::{BufferedStream, IoResult, IoError};
use std::io::IoErrorKind::EndOfFile;
use std::os::{getenv, setenv};

fn main() {
    set_default_log();

    let server = match tcp_server::Server::new("0.0.0.0:8000") {
        Ok(s) => s,
        Err(e) => panic!("{}", e),
    };
    let guard = server.run(|&: mut stream: TcpStream| -> IoResult<()> {
        let src = stream.peer_name();
        info!("Connect from {:?}", src);

        let mut stream = BufferedStream::new(stream);
        loop {
            let line = match stream.read_line() {
                Ok(l) => l,
                Err(IoError{ kind: EndOfFile, ..}) => break,
                Err(e) => {
                    warn!("{}", e);
                    return Err(e);
                }
            };
            try!(stream.write_str(line.as_slice()));
            try!(stream.flush());
        }

        info!("Disconnect from {:?}", src);
        Ok(())
    });
    if let Err(e) = guard {
        panic!("{}", e);
    }
    info!("Start echo server on {}", server.addr);
}

fn set_default_log() {
    if getenv("RUST_LOG").is_none()  {
        setenv("RUST_LOG", "info");
    }
}
