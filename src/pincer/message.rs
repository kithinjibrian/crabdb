use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub version: u8,
    pub id: String,
    #[serde(rename = "type")]
    pub typ: String,
    #[serde(default)]
    pub payload: serde_json::Value,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub nonce: Option<Vec<u8>>,
    #[serde(default)]
    pub ciphertext: Option<String>,
}
