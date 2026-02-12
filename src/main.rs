extern crate mio;
extern crate http_muncher;
extern crate sha1;
extern crate rustc_serialize;

use std::net::SocketAddr;

use mio::*;
use mio::tcp::*;
mod handshake;
mod http_parser;
mod client;

use client::WebSocketClient;




struct WebSocketServer {
    socket: TcpListener,
    clients: HashMap<Token, WebSocketClient>,
    token_counter: usize,
}

const SERVER_TOKEN: Token = Token(0);

impl Handler for WebSocketServer {
    type Timeout = usize;
    type Message = ();

    fn ready(&mut self, event_loop: &mut EventLoop<WebSocketServer>, token: Token, events: EventSet) {
        if events.is_readable() {
            match token {
                SERVER_TOKEN => {
                    let client_socket = match self.socket.accept() {
                        Ok(Some((sock, _addr))) => sock,
                        Ok(None) => unreachable!(),
                        Err(e) => {
                            println!("Accept error: {}", e);
                            return;
                        }
                    };

                    let new_token = Token(self.token_counter);
                    self.clients
                        .insert(new_token, WebSocketClient::new(client_socket));
                    self.token_counter += 1;

                    event_loop
                        .register(
                            &self.clients[&new_token].socket,
                            new_token,
                            EventSet::readable(),
                            PollOpt::edge() | PollOpt::oneshot(),
                        )
                        .unwrap();
                }
                token => {
                    let client = self.clients.get_mut(&token).unwrap();
                    client.read();
                    event_loop
                        .reregister(
                            &client.socket,
                            token,
                            client.interest,
                            PollOpt::edge() | PollOpt::oneshot(),
                        )
                        .unwrap();
                }
            }
        }

        if events.is_writable() {
            let client = self.clients.get_mut(&token).unwrap();
            client.write();
            event_loop
                .reregister(
                    &client.socket,
                    token,
                    client.interest,
                    PollOpt::edge() | PollOpt::oneshot(),
                )
                .unwrap();
        }
    }
}

fn main() {
    let address = "0.0.0.0:10000".parse::<SocketAddr>().unwrap();
    let server_socket = TcpListener::bind(&address).unwrap();

    let mut event_loop = EventLoop::new().unwrap();

    let mut server = WebSocketServer {
        token_counter: 1,
        clients: HashMap::new(),
        socket: server_socket,
    };

    event_loop
        .register(&server.socket, SERVER_TOKEN, EventSet::readable(), PollOpt::edge())
        .unwrap();
    event_loop.run(&mut server).unwrap();
}
