#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("An error occurred during the HTTP request: {0}")]
    Network(#[from] reqwest::Error),

    #[error("An error occurred during the Shift-JIS to UTF-8 conversion: {0}")]
    Decode(String),

    #[error("An error occurred during CSV parsing: {0}")]
    Csv(#[from] csv::Error),
}
