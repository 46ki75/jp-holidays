//! Generates the static `jp-holidays` JSON API into an output directory, ready
//! to be published to GitHub Pages.
//!
//! Layout produced under `<out-dir>`:
//!
//! ```text
//! .nojekyll
//! index.html
//! api/v1/holidays.json   # { "1955-01-01": "元日", ... }  all holidays
//! api/v1/{year}.json     # same shape, one year
//! api/v1/years.json      # { "years": [...], "count": N, "source": "...", "generated_at": "<ISO>" }
//! ```

use std::collections::BTreeMap;
use std::path::PathBuf;

use anyhow::{Context, Result};
use chrono::Datelike;
use clap::Parser;
use jp_holidays_lib::Client;
use serde::Serialize;

/// Source of the holiday data, recorded in `years.json`.
const SOURCE_URL: &str = "https://www8.cao.go.jp/chosei/shukujitsu/syukujitsu.csv";

/// Landing page documenting the static API.
const INDEX_HTML: &str = include_str!("index.html");

#[derive(Parser)]
#[command(about = "Generate the static jp-holidays JSON API", version)]
struct Args {
    /// Output directory for the generated site.
    #[arg(long, default_value = "dist")]
    out_dir: PathBuf,
}

/// The `api/v1/years.json` index document.
#[derive(Serialize)]
struct YearsIndex {
    /// All years that have at least one holiday, ascending.
    years: Vec<i32>,
    /// Total number of holidays across all years.
    count: usize,
    /// Upstream source of the data.
    source: &'static str,
    /// RFC 3339 timestamp of when the site was generated.
    generated_at: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let client = Client::fetch()
        .await
        .context("failed to fetch holiday data from the Cabinet Office")?;

    let generated_at = chrono::Utc::now().to_rfc3339();
    let files = render(&client, generated_at);

    for (relative, contents) in &files {
        let path = args.out_dir.join(relative);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("creating directory {}", parent.display()))?;
        }
        std::fs::write(&path, contents).with_context(|| format!("writing {}", path.display()))?;
    }

    println!(
        "Generated {} files into {}",
        files.len(),
        args.out_dir.display()
    );
    Ok(())
}

/// Renders every output file as `(relative path, contents)` pairs.
///
/// This is pure given a `Client` and timestamp, so it can be tested against the
/// bundled data without touching the network.
fn render(client: &Client, generated_at: String) -> Vec<(PathBuf, String)> {
    let mut files: Vec<(PathBuf, String)> = Vec::new();

    // Date-keyed map of every holiday, plus the same data grouped by year.
    let mut all: BTreeMap<String, &str> = BTreeMap::new();
    let mut by_year: BTreeMap<i32, BTreeMap<String, &str>> = BTreeMap::new();
    for (date, name) in client.list_holidays() {
        let key = date.format("%Y-%m-%d").to_string();
        all.insert(key.clone(), name.as_str());
        by_year
            .entry(date.year())
            .or_default()
            .insert(key, name.as_str());
    }

    files.push((PathBuf::from("api/v1/holidays.json"), to_json(&all)));

    for (year, map) in &by_year {
        files.push((PathBuf::from(format!("api/v1/{year}.json")), to_json(map)));
    }

    let index = YearsIndex {
        years: by_year.keys().copied().collect(),
        count: all.len(),
        source: SOURCE_URL,
        generated_at,
    };
    files.push((PathBuf::from("api/v1/years.json"), to_json(&index)));

    files.push((PathBuf::from(".nojekyll"), String::new()));
    files.push((PathBuf::from("index.html"), INDEX_HTML.to_string()));

    files
}

/// Serializes a value to pretty JSON with a trailing newline.
fn to_json<T: Serialize>(value: &T) -> String {
    // The inputs here are plain maps/structs that always serialize cleanly.
    let mut s =
        serde_json::to_string_pretty(value).expect("holiday data should always serialize to JSON");
    s.push('\n');
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    fn files() -> BTreeMap<PathBuf, String> {
        render(&Client::new(), "2026-01-01T00:00:00+00:00".to_string())
            .into_iter()
            .collect()
    }

    #[test]
    fn emits_expected_paths() {
        let files = files();
        assert!(files.contains_key(&PathBuf::from("api/v1/holidays.json")));
        assert!(files.contains_key(&PathBuf::from("api/v1/years.json")));
        assert!(files.contains_key(&PathBuf::from("api/v1/1955.json")));
        assert!(files.contains_key(&PathBuf::from(".nojekyll")));
        assert!(files.contains_key(&PathBuf::from("index.html")));
    }

    #[test]
    fn holidays_json_is_a_date_keyed_map() {
        let files = files();
        let json = &files[&PathBuf::from("api/v1/holidays.json")];
        let map: BTreeMap<String, String> = serde_json::from_str(json).unwrap();
        assert_eq!(map.get("1955-01-01").map(String::as_str), Some("元日"));
    }

    #[test]
    fn per_year_file_only_contains_that_year() {
        let files = files();
        let json = &files[&PathBuf::from("api/v1/1955.json")];
        let map: BTreeMap<String, String> = serde_json::from_str(json).unwrap();
        assert!(!map.is_empty());
        assert!(map.keys().all(|k| k.starts_with("1955-")));
    }

    #[test]
    fn years_index_is_consistent() {
        let files = files();
        let index: serde_json::Value =
            serde_json::from_str(&files[&PathBuf::from("api/v1/years.json")]).unwrap();
        let all: BTreeMap<String, String> =
            serde_json::from_str(&files[&PathBuf::from("api/v1/holidays.json")]).unwrap();

        assert_eq!(index["count"].as_u64().unwrap() as usize, all.len());
        assert_eq!(index["source"].as_str().unwrap(), SOURCE_URL);
        assert_eq!(
            index["generated_at"].as_str().unwrap(),
            "2026-01-01T00:00:00+00:00"
        );
        let years = index["years"].as_array().unwrap();
        assert_eq!(years.first().unwrap().as_i64().unwrap(), 1955);
    }
}
