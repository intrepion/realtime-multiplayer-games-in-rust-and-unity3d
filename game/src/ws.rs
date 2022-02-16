use std::net::TcpStream;
use tungstenite::{client::connect, stream::MaybeTlsStream, WebSocket};

pub struct Connection {
    socket: Option<WebSocket<MaybeTlsStream<TcpStream>>>,
}

impl Connection {
    pub fn new() -> Self {
        Self { socket: None }
    }

    pub fn connect(&mut self, url: &str) {
        if let Ok((mut socket, _)) = connect(url) {
            if let MaybeTlsStream::Plain(s) = socket.get_mut() {
                s.set_nonblocking(true).unwrap();
            }

            self.socket = Some(socket);
        }
    }
}
