//! Defines crate-wide error types for use throughout the application.

/// Error type for this crate.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Occurs when sending HTTP requests fails.
    #[error("HTTP リクエスト中にエラーが発生: {0}")]
    Http(String),

    /// Occurs when reading response body fails.
    #[error("レスポンスボディの読み取りに失敗: {0}")]
    BodyRead(String),

    /// Occurs when parsing CSV fails.
    #[error("CSV のパースに失敗: {0}")]
    Parse(String),

    /// Occurs when date format is invalid.
    #[error("不正な日付: {0}")]
    InvalidDate(String),
}
