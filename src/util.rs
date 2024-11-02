pub async fn fetch_csv(url: &str) -> Result<Vec<u8>, crate::error::Error> {
    let response = reqwest::get(url).await?;
    let body = response.bytes().await?;
    Ok(body.to_vec())
}

pub fn to_utf8(bytes: &[u8]) -> Result<String, crate::error::Error> {
    println!("Debug: Input bytes: {:?}", bytes);
    let (decoded_str, _, had_errors) = encoding_rs::SHIFT_JIS.decode(bytes);
    if had_errors {
        Err(crate::error::Error::Decode(
            "Shift-JIS to UTF-8 decoding error".to_string(),
        ))
    } else {
        Ok(decoded_str.into_owned())
    }
}
