use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct ChunkPayload {
    pub chunk: Option<String>,
    pub status: i8,
}