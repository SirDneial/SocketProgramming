use base64::engine::general_purpose::STANDARD;
use base64::Engine;

pub fn encode_to_base64(content: String) -> String {
    STANDARD.encode(content)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_to_base64() {
        let input = "ALBNM, PROD001, 12, 2023-01-01";
        let expected = "QUxCTk0sIFBST0QwMDEsIDEyLCAyMDIzLTAxLTAx";
        assert_eq!(encode_to_base64(input.to_string()), expected);
    }
}
