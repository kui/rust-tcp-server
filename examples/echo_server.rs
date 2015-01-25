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
    match server.run(handle_stream) {
        Ok(_) => info!("Start echo server on {}", server.addr),
        Err(e) => panic!("{}", e),
    };
}

fn handle_stream(mut stream: TcpStream) -> IoResult<()> {
    let src = stream.peer_name()
        .map(|a| a.to_string())
        .unwrap_or_else(|_| String::from_str("<Unknown Source>"));
    info!("[{}] Connect", src);

    match handle_stream_main(BufferedStream::new(stream)) {
        Ok(()) => {
            info!("[{}] Disconnect", src);
            Ok(())
        }
        Err(e) => {
            warn!("[{}] {}", src, e);
            Err(e)
        }
    }
}

fn handle_stream_main(mut stream: BufferedStream<TcpStream>) -> IoResult<()> {
    loop {
        let line = match stream.read_line() {
            Ok(l) => l,
            Err(IoError{ kind: EndOfFile, ..}) => return Ok(()),
            Err(e) => return Err(e)
        };

        try!(stream.write_str(line.as_slice()));
        try!(stream.flush());
    }
}

fn set_default_log() {
    if getenv("RUST_LOG").is_none()  {
        setenv("RUST_LOG", "info");
    }
}
