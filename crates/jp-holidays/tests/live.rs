//! Live tests for the generator.
//!
//! These fetch the holiday data from the Cabinet Office over the network and
//! run it through [`jp_holidays::render`], asserting the generated API is
//! complete and well-formed against the *live* upstream shape — they catch
//! upstream format changes that the bundled-data unit tests cannot. They are
//! `#[ignore]`d so a plain `cargo test` never reaches the network; opt in with:
//!
//! ```sh
//! cargo test -p jp-holidays -- --ignored
//! ```

use std::collections::BTreeMap;
use std::path::PathBuf;

use jp_holidays::{SOURCE_URL, render};
use jp_holidays_lib::Client;

/// Fetches live data and renders it into a path-keyed map of files.
async fn live_files() -> BTreeMap<PathBuf, String> {
    let client = Client::fetch()
        .await
        .expect("fetching live holiday data should succeed");
    render(&client, "2026-01-01T00:00:00+00:00".to_string())
        .into_iter()
        .collect()
}

#[tokio::test]
#[ignore = "live: hits the Cabinet Office network endpoint"]
async fn live_render_emits_a_complete_api() {
    let files = live_files().await;
    for path in [
        "api/v1/holidays.json",
        "api/v1/years.json",
        "api/v1/openapi.json",
        "api/v1/docs.html",
        ".nojekyll",
    ] {
        assert!(files.contains_key(&PathBuf::from(path)), "missing {path}");
    }
    // The generator never emits the landing page — that's the React app's.
    assert!(!files.contains_key(&PathBuf::from("index.html")));
}

#[tokio::test]
#[ignore = "live: hits the Cabinet Office network endpoint"]
async fn live_holidays_json_is_a_date_keyed_map() {
    let files = live_files().await;
    let all: BTreeMap<String, String> =
        serde_json::from_str(&files[&PathBuf::from("api/v1/holidays.json")]).unwrap();

    assert!(!all.is_empty());
    // A settled, historical holiday that must always be present.
    assert_eq!(all.get("1955-01-01").map(String::as_str), Some("元日"));
    // Every key is a YYYY-MM-DD date.
    assert!(all.keys().all(|k| k.len() == 10 && k.as_bytes()[4] == b'-'));
}

#[tokio::test]
#[ignore = "live: hits the Cabinet Office network endpoint"]
async fn live_years_index_and_per_year_files_are_consistent() {
    let files = live_files().await;
    let index: serde_json::Value =
        serde_json::from_str(&files[&PathBuf::from("api/v1/years.json")]).unwrap();
    let all: BTreeMap<String, String> =
        serde_json::from_str(&files[&PathBuf::from("api/v1/holidays.json")]).unwrap();

    assert_eq!(index["count"].as_u64().unwrap() as usize, all.len());
    assert_eq!(index["source"].as_str().unwrap(), SOURCE_URL);

    // Every listed year has a per-year file holding only that year's dates.
    let years = index["years"].as_array().unwrap();
    assert!(!years.is_empty());
    for year in years {
        let year = year.as_i64().unwrap();
        let map: BTreeMap<String, String> =
            serde_json::from_str(&files[&PathBuf::from(format!("api/v1/{year}.json"))]).unwrap();
        assert!(!map.is_empty());
        assert!(map.keys().all(|k| k.starts_with(&format!("{year}-"))));
    }
}
