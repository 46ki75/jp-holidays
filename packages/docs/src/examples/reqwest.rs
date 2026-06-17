// reqwest = { version = "0.12", features = ["json"] }
use std::collections::HashMap;

#[tokio::main]
async fn main() -> reqwest::Result<()> {
    let url = "https://46ki75.github.io/jp-holidays/api/v1/2026.json";
    let holidays: HashMap<String, String> = reqwest::get(url).await?.json().await?;

    // 日付（YYYY-MM-DD）から祝日名を引く
    println!("{:?}", holidays.get("2026-01-01")); // Some("元日")
    Ok(())
}
