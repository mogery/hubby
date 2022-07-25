use hubby_macros::{register_status_packet, generate_status_handler, identify_packet};
use serde::{Deserialize, Serialize};

use crate::connection::Connection;

use super::{HandleError, IdentifiedPacket};

#[derive(Serialize)]
#[identify_packet(0x00)]
pub struct StatusResponse {
    pub status: String,
}

#[derive(Serialize)]
#[identify_packet(0x01)]
pub struct PingResponse {
    pub payload: i64,
}

#[derive(Deserialize, Debug)]
pub struct StatusRequest {}

#[derive(Deserialize, Debug)]
pub struct PingRequest {
    pub payload: i64
}

#[register_status_packet(0x00)]
async fn handle_status_request(conn: &mut Connection<'_>, _packet: StatusRequest) -> Result<(), HandleError> {
    println!("status requested");

    let res = StatusResponse { 
        status: r#"{
    "version": {
        "name": "1.19",
        "protocol": 759
    },
    "players": {
        "max": 100,
        "online": 0,
        "sample": []
    },
    "description": {
        "text": "Hubby"
    },
    "previewsChat": true
}"#.to_string(),
    };

    conn.send_packet(res).await;
    
    Ok(())
}

#[register_status_packet(0x01)]
async fn handle_ping_request(conn: &mut Connection<'_>, packet: PingRequest) -> Result<(), HandleError> {
    println!("ping requested");

    conn.send_packet(PingResponse {
        payload: packet.payload,
    }).await;

    Ok(())
}

generate_status_handler!();