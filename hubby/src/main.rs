mod varint;
mod connection;
mod packets;

use connection::Connection;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("0.0.0.0:2346").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let conn = Connection {
                socket: &mut socket,
                state: connection::ConnectionState::Handshaking,
            };

            conn.listen().await;
        });
    }
}