mod date_format;
mod error;
mod holiday;
mod util;

#[tokio::main]
async fn main() -> Result<(), error::Error> {
    let raw_csv_bytes =
        util::fetch_csv("https://www8.cao.go.jp/chosei/shukujitsu/syukujitsu.csv").await?;

    let csv = util::to_utf8(&raw_csv_bytes)?;

    let holidays = holiday::Holiday::deserialize_from_csv(&csv)?;

    println!("{:?}", holidays);

    Ok(())
}
