use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use http_muncher::Parser;
use mio::{EventSet, PollOpt};
use mio::tcp::TcpStream;

use handshake::build_handshake_response;
use http_parser::HttpParser;

#[derive(PartialEq)]
pub enum ClientState {
    AwaitingHandshake,
    HandshakeResponse,
    Connected,
}

pub struct WebSocketClient {
    pub socket: TcpStream,
    headers: Rc<RefCell<HashMap<String, String>>>,
    http_parser: Parser<HttpParser>,
    pub interest: EventSet,
    state: ClientState,
}

impl WebSocketClient {
    pub fn new(socket: TcpStream) -> WebSocketClient {
        let headers = Rc::new(RefCell::new(HashMap::new()));

        WebSocketClient {
            socket: socket,
            headers: headers.clone(),
            http_parser: Parser::request(HttpParser::new(headers.clone())),
            interest: EventSet::readable(),
            state: ClientState::AwaitingHandshake,
        }
    }

    pub fn write(&mut self) {
        let headers = self.headers.borrow();
        let response = build_handshake_response(&headers);
        self.socket.try_write(response.as_bytes()).unwrap();

        // Change the state
        self.state = ClientState::Connected;

        self.interest.remove(EventSet::writable());
        self.interest.insert(EventSet::readable());
    }

    pub fn read(&mut self) {
        loop {
            let mut buf = [0; 2048];
            match self.socket.try_read(&mut buf) {
                Err(e) => {
                    println!("Error while reading socket: {:?}", e);
                    return;
                }
                Ok(None) =>
                    // Socket buffer has got no more bytes.
                    break,
                Ok(Some(_len)) => {
                    self.http_parser.parse(&buf);
                    if self.http_parser.is_upgrade() {
                        // Change the current state
                        self.state = ClientState::HandshakeResponse;

                        // Change current interest to `Writable`
                        self.interest.remove(EventSet::readable());
                        self.interest.insert(EventSet::writable());
                        break;
                    }
                }
            }
        }
    }
}
