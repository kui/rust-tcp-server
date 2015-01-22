#![allow(unstable)]

#[macro_use] extern crate log;

use std::thread;
use std::io::{IoResult, Listener, Acceptor};
use std::io::net::ip::{ToSocketAddr, SocketAddr};
use std::io::net::tcp::{TcpStream, TcpListener};
use std::thread::JoinGuard;
use std::sync::Arc;

#[derive(Show, Copy)]
pub struct Server {
    pub addr: SocketAddr
}

impl Server {
    pub fn new<A: ToSocketAddr>(addr: A) -> IoResult<Server> {
        addr.to_socket_addr().map(|a| Server { addr: a })
    }

    pub fn run<H: Handler>(&self, handler: H) -> IoResult<JoinGuard<'static, ()>> {
        let tname = format!("TCP Listener {}", self.addr);
        debug!("Start: {}", tname);

        let handler = Arc::new(handler);
        let mut acceptor = try!(TcpListener::bind(self.addr)).listen();
        let guard = thread::Builder::new().name(tname).scoped(move || {
            for stream in acceptor.incoming() {
                match stream {
                    Ok(tcp) => { Server::handle_stream(handler.clone(), tcp); }
                    Err(e) => warn!("{}", e),
                }
            }
        });
        Ok(guard)
    }

    fn handle_stream<H: Handler>(handler: Arc<H>, mut tcp: TcpStream) {
        let src = tcp.peer_name()
            .map(|s| { format!("{}", s) })
            .unwrap_or(String::from_str("<Unknown Peer Name>"));
        debug!("Connect from {}", src);
        let tname = format!("TCP Stream Handler {}", src);
        thread::Builder::new().name(tname).spawn(move || {
            match handler.handle(tcp) {
                Ok(_) => debug!("Disconnect from {}", src),
                Err(e) => warn!("{}", e),
            }
        });
    }
}

pub trait Handler: Send + Sync {
    fn handle(&self, s: TcpStream) -> IoResult<()>;
}

impl<F> Handler for F
where F: Fn(TcpStream) -> IoResult<()>, F: Send, F: Sync {
    fn handle(&self, s: TcpStream) -> IoResult<()> {
        (*self)(s)
    }
}
