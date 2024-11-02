mod date_format;
mod error;
mod holiday;
mod response;
mod util;

#[tokio::main]
async fn main() -> Result<(), error::Error> {
    let raw_csv_bytes =
        util::fetch_csv("https://www8.cao.go.jp/chosei/shukujitsu/syukujitsu.csv").await?;

    let csv = util::to_utf8(&raw_csv_bytes)?;

    let holidays = holiday::Holiday::deserialize_from_csv(&csv)?;

    let responses: Vec<response::Response> = holidays.into_iter().map(Into::into).collect();

    println!("{:?}", responses);

    Ok(())
}
