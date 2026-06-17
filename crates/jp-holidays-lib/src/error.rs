//! Defines crate-wide error types.

/// Error type for this crate.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The HTTP request to fetch holiday data failed.
    ///
    /// Only produced by [`Client::fetch`](crate::Client::fetch), available
    /// under the `fetch` feature.
    #[cfg(feature = "fetch")]
    #[error("failed to fetch holiday data over HTTP: {0}")]
    Http(#[from] reqwest::Error),

    /// The CSV was structurally malformed (e.g. a row was missing the date or
    /// name column).
    #[error("malformed holiday CSV: {0}")]
    MalformedCsv(String),

    /// A date in the CSV could not be parsed.
    #[error("failed to parse date in holiday CSV: {0}")]
    ParseDate(#[from] chrono::ParseError),

    /// The supplied year/month/day do not form a valid calendar date.
    #[error("invalid date: {year}-{month}-{day}")]
    InvalidDate {
        /// Year component.
        year: i32,
        /// Month component.
        month: u32,
        /// Day component.
        day: u32,
    },
}
