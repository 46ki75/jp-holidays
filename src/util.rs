pub async fn fetch_csv(url: &str) -> Result<Vec<u8>, crate::error::Error> {
    let response = reqwest::get(url).await?;
    let body = response.bytes().await?;
    Ok(body.to_vec())
}

pub fn to_utf8(bytes: &[u8]) -> Result<String, crate::error::Error> {
    let (decoded_str, _, had_errors) = encoding_rs::SHIFT_JIS.decode(bytes);
    if had_errors {
        Err(crate::error::Error::Decode(
            "Shift-JIS to UTF-8 decoding error".to_string(),
        ))
    } else {
        Ok(decoded_str.into_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_utf8_valid_shift_jis() {
        let shift_jis_bytes = vec![0x82, 0xa0, 0x82, 0xa2, 0x82, 0xa4];
        let result = to_utf8(&shift_jis_bytes);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "あいう");
    }

    #[test]
    fn test_to_utf8_invalid_shift_jis() {
        let invalid_shift_jis_bytes = vec![0x82, 0xa0, 0xff, 0x82, 0xa4];
        let result = to_utf8(&invalid_shift_jis_bytes);
        assert!(result.is_err());
    }

    #[test]
    fn test_to_utf8_empty_input() {
        let empty_bytes: Vec<u8> = vec![];
        let result = to_utf8(&empty_bytes);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "");
    }
}
