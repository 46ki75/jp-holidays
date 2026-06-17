//! Live retrieval of holiday data from the Cabinet Office of Japan.
//!
//! This module is only compiled when the `fetch` feature is enabled.

/// Endpoint for the Cabinet Office of Japan's official holiday CSV (Shift-JIS).
const ENDPOINT: &str = "https://www8.cao.go.jp/chosei/shukujitsu/syukujitsu.csv";

/// Fetches the latest holiday CSV from the Cabinet Office and decodes it from
/// Shift-JIS into a UTF-8 `String`.
pub(crate) async fn fetch_csv() -> Result<String, crate::error::Error> {
    let bytes = reqwest::Client::new()
        .get(ENDPOINT)
        .send()
        .await?
        .bytes()
        .await?;

    let (cow, _, _) = encoding_rs::SHIFT_JIS.decode(&bytes);
    Ok(cow.into_owned())
}
