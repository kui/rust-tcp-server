#![allow(unstable)]

extern crate tcp_server;

use std::io::net::tcp::TcpStream;
use std::io::IoResult;

fn main() {
    let server = match tcp_server::Server::new("0.0.0.0:8000") {
        Ok(s) => s,
        Err(e) => panic!("{}", e),
    };
    let guard = server.run(|&: mut s: TcpStream| -> IoResult<()> {
        println!("Connect from {:?}", s.peer_name());
        try!(writeln!(&mut s, "Hello!"));
        Ok(())
    });
    match guard {
        Ok(_) => println!("Start greet server on {}", server.addr),
        Err(e) => panic!("{}", e),
    };

    // this guard will `join` when it call `drop`
}
