pub mod handshaking;
pub mod status;

pub enum HandleError {
    SerdeMCJE(serde_mcje::Error),
    Unimplemented(i32),
    BadPacket(String),
}

pub trait IdentifiedPacket {
    const ID: i32;
}