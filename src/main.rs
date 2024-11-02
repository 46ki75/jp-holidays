mod error;
mod fetch;

#[tokio::main]
async fn main() -> Result<(), error::Error> {
    let raw_csv =
        fetch::fetch_csv("https://www8.cao.go.jp/chosei/shukujitsu/syukujitsu.csv").await?;

    println!("{}", raw_csv);

    Ok(())
}
