use std::io::ErrorKind;

use serde::Serialize;
use serde_mcje::to_vec;
use tokio::{net::TcpStream, io::{AsyncReadExt, AsyncWriteExt}};
use crate::{varint::*, packets::{self, HandleError, IdentifiedPacket}};

pub enum ConnectionState {
    Handshaking,
    Status,
    Login,
    Play
}

pub struct Connection<'a> {
    pub socket: &'a mut TcpStream,
    pub state: ConnectionState,
}

impl Connection<'_> {
    pub async fn listen(mut self) {
        loop {
            let len = match read_varint_tcp(self.socket).await {
                Ok(x) => x,
                Err(VarIntError::Io(e)) => {
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
            match self.socket.read_exact(&mut vec).await {
                Ok(_) => {},
                Err(e) => {
                    eprintln!("failed to read packet; err = {:?}", e);
                    return;
                }
            };

            let mut buf = &mut vec[..];

            let (id, id_len) = match read_varint(buf) {
                Ok(x) => x,
                Err(e) => {
                    eprintln!("failed to read packet ID from socket; err = {:?}", e);
                    return;
                }
            };

            buf = &mut buf[id_len..];

            println!("Received packet ID {} with content {:?}", id, buf);

            let handle_result = match self.state {
                ConnectionState::Handshaking => packets::handshaking::handle(&mut self, id, buf).await,
                ConnectionState::Status => packets::status::handle(&mut self, id, buf).await,
                _ => panic!("idk")
            };

            match handle_result {
                Ok(_) => {},
                Err(e) => match e {
                    HandleError::SerdeMCJE(e) => {
                        eprintln!("failed to parse packet; err = {:#?}", e);
                        return;
                    },
                    HandleError::Unimplemented(id) => eprintln!("packet ID {} unimplemented", id),
                    HandleError::BadPacket(e) => {
                        eprintln!("invalid packet received; {}", e);
                        return;
                    }
                }
            };
        }
    }

    pub fn switch_state(&mut self, new_state: ConnectionState) {
        self.state = new_state;
    }

    pub async fn send_packet<T: Serialize + IdentifiedPacket>(&mut self, packet: T) {
        let id = write_varint(T::ID);
        let pak = to_vec(&packet).unwrap();

        self.socket.write_all(&write_varint((id.len() + pak.len()) as i32)).await.unwrap();
        self.socket.write_all(&id).await.unwrap();
        self.socket.write_all(&pak).await.unwrap();
    }
}