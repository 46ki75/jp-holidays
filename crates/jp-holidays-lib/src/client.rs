//! The crate entry point: [`Client`].

use chrono::{Datelike, NaiveDate};
use std::collections::BTreeMap;

/// Holiday data for Japan, queried through a set of convenience methods.
///
/// A `Client` is constructed in one of three ways:
///
/// - [`Client::new`] — synchronous, backed by the holiday data bundled into the
///   crate at build time. No network access and no async runtime required.
/// - [`Client::from_csv`] — parse a UTF-8 CSV string in the Cabinet Office
///   format that you supply yourself.
/// - [`Client::fetch`] — *(feature `fetch`)* download the latest data from the
///   Cabinet Office of Japan at runtime.
pub struct Client {
    data: BTreeMap<NaiveDate, String>,
}

impl Client {
    /// Creates a client from the holiday data bundled into the crate at build
    /// time.
    ///
    /// This is synchronous and requires neither network access nor an async
    /// runtime. The bundled data is refreshed periodically and shipped with new
    /// releases; for guaranteed up-to-date data use [`Client::fetch`].
    ///
    /// ## Example
    ///
    /// ```
    #[doc = include_str!("../examples/basic.rs")]
    /// ```
    pub fn new() -> Self {
        // The bundled CSV is validated by a unit test, so parsing cannot fail.
        let data = crate::parse::parse_csv(Self::BUNDLED_CSV)
            .expect("bundled holiday CSV should always parse");
        Self { data }
    }

    /// The UTF-8 holiday CSV bundled into the crate at build time.
    const BUNDLED_CSV: &'static str = include_str!("data/syukujitsu.csv");

    /// Creates a client by parsing a UTF-8 CSV string in the Cabinet Office
    /// format (`YYYY/M/D,name` rows with a header line).
    ///
    /// ## Errors
    ///
    /// Returns an error if the CSV is structurally malformed or contains an
    /// unparseable date.
    pub fn from_csv(csv: &str) -> Result<Self, crate::error::Error> {
        Ok(Self {
            data: crate::parse::parse_csv(csv)?,
        })
    }

    /// Creates a client by downloading the latest holiday data from the Cabinet
    /// Office of Japan.
    ///
    /// Requires the `fetch` feature.
    ///
    /// ## Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    #[cfg(feature = "fetch")]
    pub async fn fetch() -> Result<Self, crate::error::Error> {
        let csv = crate::fetch::fetch_csv().await?;
        Self::from_csv(&csv)
    }

    /// Returns every published holiday as a map sorted by date.
    ///
    /// ## Example
    ///
    /// ```
    #[doc = include_str!("../examples/list_holidays.rs")]
    /// ```
    pub fn list_holidays(&self) -> &BTreeMap<NaiveDate, String> {
        &self.data
    }

    /// Returns the holiday name for `date`, or `None` if it is not a holiday.
    ///
    /// ## Example
    ///
    /// ```
    #[doc = include_str!("../examples/get_holiday.rs")]
    /// ```
    pub fn get_holiday(&self, date: NaiveDate) -> Option<&str> {
        self.data.get(&date).map(|s| s.as_str())
    }

    /// Returns the holiday name for the given year/month/day.
    ///
    /// ## Errors
    ///
    /// Returns [`Error::InvalidDate`](crate::error::Error::InvalidDate) if the
    /// arguments do not form a valid calendar date.
    ///
    /// ## Example
    ///
    /// ```
    #[doc = include_str!("../examples/get_holiday_ymd.rs")]
    /// ```
    pub fn get_holiday_ymd(
        &self,
        year: i32,
        month: u32,
        day: u32,
    ) -> Result<Option<&str>, crate::error::Error> {
        Ok(self.get_holiday(to_date(year, month, day)?))
    }

    /// Returns whether `date` is a holiday.
    ///
    /// ## Example
    ///
    /// ```
    #[doc = include_str!("../examples/is_holiday.rs")]
    /// ```
    pub fn is_holiday(&self, date: NaiveDate) -> bool {
        self.data.contains_key(&date)
    }

    /// Returns whether the given year/month/day is a holiday.
    ///
    /// ## Errors
    ///
    /// Returns [`Error::InvalidDate`](crate::error::Error::InvalidDate) if the
    /// arguments do not form a valid calendar date.
    ///
    /// ## Example
    ///
    /// ```
    #[doc = include_str!("../examples/is_holiday_ymd.rs")]
    /// ```
    pub fn is_holiday_ymd(
        &self,
        year: i32,
        month: u32,
        day: u32,
    ) -> Result<bool, crate::error::Error> {
        Ok(self.is_holiday(to_date(year, month, day)?))
    }

    /// Returns whether `date` is a day off — a holiday or a weekend (Sat/Sun).
    ///
    /// ## Example
    ///
    /// ```
    #[doc = include_str!("../examples/is_day_off.rs")]
    /// ```
    pub fn is_day_off(&self, date: NaiveDate) -> bool {
        matches!(date.weekday(), chrono::Weekday::Sat | chrono::Weekday::Sun)
            || self.is_holiday(date)
    }

    /// Returns whether the given year/month/day is a day off — a holiday or a
    /// weekend (Sat/Sun).
    ///
    /// ## Errors
    ///
    /// Returns [`Error::InvalidDate`](crate::error::Error::InvalidDate) if the
    /// arguments do not form a valid calendar date.
    ///
    /// ## Example
    ///
    /// ```
    #[doc = include_str!("../examples/is_day_off_ymd.rs")]
    /// ```
    pub fn is_day_off_ymd(
        &self,
        year: i32,
        month: u32,
        day: u32,
    ) -> Result<bool, crate::error::Error> {
        Ok(self.is_day_off(to_date(year, month, day)?))
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

/// Builds a [`NaiveDate`] from components, mapping an invalid date to
/// [`Error::InvalidDate`](crate::error::Error::InvalidDate).
fn to_date(year: i32, month: u32, day: u32) -> Result<NaiveDate, crate::error::Error> {
    NaiveDate::from_ymd_opt(year, month, day).ok_or(crate::error::Error::InvalidDate {
        year,
        month,
        day,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bundled_csv_parses() {
        // Guards the `expect` in `Client::new`.
        let client = Client::new();
        assert!(!client.list_holidays().is_empty());
    }

    #[test]
    fn get_holiday_known_date() {
        let client = Client::new();
        assert_eq!(client.get_holiday_ymd(1955, 1, 1).unwrap(), Some("元日"));
    }

    #[test]
    fn get_holiday_unknown_date() {
        let client = Client::new();
        assert_eq!(client.get_holiday_ymd(1955, 1, 2).unwrap(), None);
    }

    #[test]
    fn is_holiday_true_and_false() {
        let client = Client::new();
        assert!(client.is_holiday_ymd(1955, 5, 5).unwrap());
        assert!(!client.is_holiday_ymd(1955, 5, 4).unwrap());
    }

    #[test]
    fn invalid_date_errors() {
        let client = Client::new();
        assert!(client.get_holiday_ymd(1955, 2, 30).is_err());
    }

    #[test]
    fn is_day_off_holiday_weekend_and_weekday() {
        let client = Client::new();
        assert!(client.is_day_off_ymd(1955, 1, 1).unwrap()); // holiday
        assert!(client.is_day_off_ymd(1955, 1, 8).unwrap()); // Saturday
        assert!(!client.is_day_off_ymd(1955, 1, 5).unwrap()); // ordinary weekday
    }

    #[test]
    fn from_csv_roundtrip() {
        let client = Client::from_csv("h,h\n2020/1/1,元日\n").unwrap();
        assert_eq!(client.get_holiday_ymd(2020, 1, 1).unwrap(), Some("元日"));
    }
}
