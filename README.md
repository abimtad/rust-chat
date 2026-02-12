# rust-chat

Minimal WebSocket server prototype built on `mio` and `http-muncher`. It performs a
basic HTTP Upgrade handshake, then keeps the connection open for future work.

This project focuses on a small, modular event-loop driven server that shows how
to:

- parse HTTP headers for a WebSocket upgrade
- generate the `Sec-WebSocket-Accept` response key
- register and reregister sockets with `mio`

## Project layout

- `src/main.rs` wires modules and starts the event loop
- `src/server.rs` manages the listener and client registry
- `src/client.rs` handles read/write for a single connection
- `src/http_parser.rs` parses HTTP headers from the client
- `src/handshake.rs` builds the handshake response

## Build

```bash
cargo build
```

## Run

```bash
cargo run
```

The server listens on `0.0.0.0:10000`.

## Notes

- This is a prototype: it only performs the handshake and does not implement
	WebSocket frames yet.
- Error handling is minimal and intended for clarity.
