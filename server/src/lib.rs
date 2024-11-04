use base64::engine::general_purpose::STANDARD;
use base64::Engine;

pub fn decode_base64(encoded: String) -> Vec<u8> {
    STANDARD.decode(encoded).expect("Failed to decode Base64 content")
}
