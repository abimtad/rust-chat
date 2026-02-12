use std::collections::HashMap;
use std::fmt;

use rustc_serialize::base64::{ToBase64, STANDARD};

fn gen_key(key: &str) -> String {
    let mut m = sha1::Sha1::new();
    let mut buf = [0u8; 20];

    m.update(key.as_bytes());
    m.update("258EAFA5-E914-47DA-95CA-C5AB0DC85B11".as_bytes());

    m.output(&mut buf);

    buf.to_base64(STANDARD)
}

pub fn build_handshake_response(headers: &HashMap<String, String>) -> String {
    let response_key = gen_key(headers.get("Sec-WebSocket-Key").unwrap());
    fmt::format(format_args!(
        "HTTP/1.1 101 Switching Protocols\r\n\
         Connection: Upgrade\r\n\
         Sec-WebSocket-Accept: {}\r\n\
         Upgrade: websocket\r\n\r\n",
        response_key
    ))
}
