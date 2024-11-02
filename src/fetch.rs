pub async fn fetch_csv(url: &str) -> Result<String, crate::error::Error> {
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}
