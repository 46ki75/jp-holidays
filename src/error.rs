#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("An error occurred during the HTTP request: {0}")]
    Network(#[from] reqwest::Error),
}
