use std::collections::HashMap;

use mio::{EventLoop, EventSet, Handler, PollOpt, Token};
use mio::tcp::TcpListener;

use client::WebSocketClient;

pub struct WebSocketServer {
    pub socket: TcpListener,
    clients: HashMap<Token, WebSocketClient>,
    token_counter: usize,
}

pub const SERVER_TOKEN: Token = Token(0);

impl WebSocketServer {
    pub fn new(socket: TcpListener) -> WebSocketServer {
        WebSocketServer {
            socket: socket,
            clients: HashMap::new(),
            token_counter: 1,
        }
    }
}

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
