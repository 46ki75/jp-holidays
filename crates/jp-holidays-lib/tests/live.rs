//! Live tests for the network `fetch` path.
//!
//! These hit the Cabinet Office network endpoint. They are compiled only with
//! the `fetch` feature and `#[ignore]`d so a plain `cargo test` never reaches
//! the network; opt in with:
//!
//! ```sh
//! cargo test -p jp-holidays-lib --features fetch -- --ignored
//! ```
#![cfg(feature = "fetch")]

use jp_holidays_lib::Client;

#[tokio::test]
#[ignore = "live: hits the Cabinet Office network endpoint"]
async fn live_get_holiday_known_date() {
    let client = Client::fetch().await.unwrap();
    assert_eq!(client.get_holiday_ymd(1955, 1, 1).unwrap(), Some("元日"));
}

#[tokio::test]
#[ignore = "live: hits the Cabinet Office network endpoint"]
async fn live_get_holiday_unknown_date() {
    let client = Client::fetch().await.unwrap();
    assert_eq!(client.get_holiday_ymd(1955, 1, 2).unwrap(), None);
}

#[tokio::test]
#[ignore = "live: hits the Cabinet Office network endpoint"]
async fn live_is_holiday() {
    let client = Client::fetch().await.unwrap();
    assert!(client.is_holiday_ymd(1955, 5, 5).unwrap());
    assert!(!client.is_holiday_ymd(1955, 5, 4).unwrap());
}

#[tokio::test]
#[ignore = "live: hits the Cabinet Office network endpoint"]
async fn live_is_day_off() {
    let client = Client::fetch().await.unwrap();
    assert!(client.is_day_off_ymd(1955, 1, 1).unwrap()); // holiday
    assert!(client.is_day_off_ymd(1955, 1, 8).unwrap()); // Saturday
    assert!(!client.is_day_off_ymd(1955, 1, 5).unwrap()); // weekday
}

#[tokio::test]
#[ignore = "live: hits the Cabinet Office network endpoint"]
async fn live_matches_bundled_for_historical_dates() {
    // The live data and the bundled data must agree on settled, historical
    // holidays.
    let fetched = Client::fetch().await.unwrap();
    let bundled = Client::new();
    let cutoff = chrono::NaiveDate::from_ymd_opt(2020, 1, 1).unwrap();
    for (date, name) in bundled.list_holidays().range(..cutoff) {
        assert_eq!(fetched.get_holiday(*date), Some(name.as_str()));
    }
}
