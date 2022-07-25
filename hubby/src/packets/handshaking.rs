use hubby_macros::{register_handshaking_packet, generate_handshaking_handler};
use serde::Deserialize;
use serde_mcje::types::VarInt;

use crate::connection::{Connection, ConnectionState};

use super::HandleError;

#[derive(Deserialize, Debug)]
pub struct Handshake {
    pub protocol_version: VarInt,
    pub server_address: String,
    pub server_port: u16,
    pub next_state: VarInt,
}

#[register_handshaking_packet(0x00)]
async fn handle_handshake(conn: &mut Connection<'_>, packet: Handshake) -> Result<(), HandleError> {
    println!("{:#?}", packet);

    conn.switch_state(match packet.next_state.0 {
        1 => ConnectionState::Status,
        2 => ConnectionState::Play,
        _ => return Err(HandleError::BadPacket(format!("invalid next_state {}", packet.next_state))),
    });

    Ok(())
}

generate_handshaking_handler!();