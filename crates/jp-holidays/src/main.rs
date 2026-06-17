//! Generates the static `jp-holidays` JSON API into an output directory, ready
//! to be published to GitHub Pages.
//!
//! Layout produced under `<out-dir>`:
//!
//! ```text
//! .nojekyll
//! api/v1/holidays.json   # { "1955-01-01": "元日", ... }  all holidays
//! api/v1/{year}.json     # same shape, one year
//! api/v1/years.json      # { "years": [...], "count": N, "source": "...", "generated_at": "<ISO>" }
//! api/v1/openapi.json    # OpenAPI 3.1 spec
//! api/v1/docs.html       # Scalar API reference
//! ```
//!
//! The rendering logic lives in the library crate (`src/lib.rs`) so it can be
//! unit- and live-tested; this binary just fetches the data, renders it, and
//! writes the files. The site's `index.html` landing page is the React app in
//! `packages/docs` (built separately and merged into the same output directory
//! at deploy time), so it is intentionally not emitted here.

use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Parser;
use jp_holidays::render;
use jp_holidays_lib::Client;

#[derive(Parser)]
#[command(about = "Generate the static jp-holidays JSON API", version)]
struct Args {
    /// Output directory for the generated site.
    #[arg(long, default_value = "dist")]
    out_dir: PathBuf,
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
