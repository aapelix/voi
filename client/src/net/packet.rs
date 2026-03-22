#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Packet {
    Audio(Vec<u8>),
}
