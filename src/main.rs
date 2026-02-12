extern crate mio;
extern crate http_muncher;
extern crate sha1;
extern crate rustc_serialize;

use std::net::SocketAddr;

use mio::{EventLoop, EventSet, PollOpt};
use mio::tcp::TcpListener;
mod handshake;
mod http_parser;
mod client;
mod server;

use server::{WebSocketServer, SERVER_TOKEN};





fn main() {
    let address = "0.0.0.0:10000".parse::<SocketAddr>().unwrap();
    let server_socket = TcpListener::bind(&address).unwrap();

    let mut event_loop = EventLoop::new().unwrap();

    let mut server = WebSocketServer::new(server_socket);

    event_loop
        .register(&server.socket, SERVER_TOKEN, EventSet::readable(), PollOpt::edge())
        .unwrap();
    event_loop.run(&mut server).unwrap();
}
