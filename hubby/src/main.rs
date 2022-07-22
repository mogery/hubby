mod varint;

use std::io::ErrorKind;

use serde_mcje::varint::read_varint;
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use varint::read_varint_tcp;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("0.0.0.0:2346").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            loop {
                let len = match read_varint_tcp(&mut socket).await {
                    Ok(x) => x,
                    Err(varint::VarIntError::Io(e)) => {
                        if e.kind() == ErrorKind::UnexpectedEof {
                            println!("Disconnected.");
                        } else {
                            eprintln!("failed to read packet length from socket; err = {:?}", e);
                        }
                        return;
                    },
                    Err(e) => {
                        eprintln!("failed to read packet length from socket; err = {:?}", e);
                        return;
                    }
                };

                let mut vec = vec![0_u8; len as usize];
                match socket.read_exact(&mut vec).await {
                    Ok(_) => {},
                    Err(e) => {
                        eprintln!("failed to read packet; err = {:?}", e);
                        return;
                    }
                };

                let mut buf = &mut vec[..];

                println!("Received with content {:?}", buf);

                let (id, id_len) = match read_varint(buf) {
                    Ok(x) => x,
                    Err(e) => {
                        eprintln!("failed to read packet ID from socket; err = {:?}", e);
                        return;
                    }
                };

                buf = &mut buf[id_len..];

                println!("Packet ID {}", id);
            }
        });
    }
}