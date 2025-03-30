#![deny(missing_docs)]

/// Endpoint URL for the Cabinet Office of Japan's official holiday data in CSV format.
const ENDPOINT: &str = "https://www8.cao.go.jp/chosei/shukujitsu/syukujitsu.csv";

#[async_trait::async_trait]
pub trait HolidayRepository: Send + Sync {
    async fn fetch_csv(&self) -> Result<bytes::Bytes, crate::error::Error>;
}

/// Implementation of the `HolidayRepository` for production use.
pub struct HolidayRepositoryImpl;

#[async_trait::async_trait]
impl HolidayRepository for HolidayRepositoryImpl {
    /// Fetches CSV data from the Cabinet Office of Japan's public file location.
    ///
    /// ## Arguments
    /// - `self` - Repository instance
    ///
    /// ## Returns
    /// - `Ok(bytes::Bytes)` - Raw CSV data (Shift-JIS)
    ///
    /// ## Errors
    /// - Failed to fetch data from the network
    /// - Failed to process the response
    async fn fetch_csv(&self) -> Result<bytes::Bytes, crate::error::Error> {
        let client = reqwest::Client::new();

        let response = client
            .get(ENDPOINT)
            .send()
            .await
            .map_err(|e| crate::error::Error::Http(e.to_string()))?;

        let response_bytes = response
            .bytes()
            .await
            .map_err(|e| crate::error::Error::BodyRead(e.to_string()))?;

        Ok(response_bytes)
    }
}

// Test double implementation of the `HolidayRepository` trait.
pub struct HolidayRepositoryStub;

#[async_trait::async_trait]
impl HolidayRepository for HolidayRepositoryStub {
    /// Reads stub CSV data from a local file (used for testing).
    ///
    /// ## Arguments
    /// - `self` - Repository instance
    ///
    /// ## Returns
    /// - `Ok(bytes::Bytes)` - Raw CSV data (Shift-JIS)
    async fn fetch_csv(&self) -> Result<bytes::Bytes, crate::error::Error> {
        let response_bytes = include_bytes!("../tests/shift-jis.csv");

        Ok(bytes::Bytes::from_static(response_bytes))
    }
}
