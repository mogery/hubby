use std::fmt::{Display, self};

use tokio::{net::TcpStream, io::AsyncReadExt};
pub use mc_varint::*;

pub async fn read_varint_tcp(socket: &mut TcpStream) -> Result<i32, VarIntError>  {
    let mut value: i32 = 0;
    let mut pos: u8 = 0;
    let mut current_byte: u8;

    loop {
        current_byte = socket.read_u8().await.or_else(|e| Err(VarIntError::Io(e)))?;

        value |= (current_byte as i32 & 0x7F) << pos;

        if (current_byte & 0x80) == 0 {
            break;
        }

        pos += 7;

        if pos >= 32 {
            return Err(VarIntError::Overflow);
        }
    }

    Ok(value)
}

pub async fn read_varlong_tcp(socket: &mut TcpStream) -> Result<i64, VarIntError>  {
    let mut value: i64 = 0;
    let mut pos: u8 = 0;
    let mut current_byte: u8;

    loop {
        current_byte = socket.read_u8().await.map_err(VarIntError::Io)?;

        value |= (current_byte as i64 & 0x7F) << pos;

        if (current_byte & 0x80) == 0 {
            break;
        }

        pos += 7;

        if pos >= 64 {
            return Err(VarIntError::Overflow);
        }
    }

    Ok(value)
}