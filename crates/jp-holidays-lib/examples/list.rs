use jp_holidays_lib::client::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::init().await?;

    // 公開されている祝日をすべて取得します
    let holidays = client.list_holidays();

    // 今回の例では10日分だけ出力
    for (date, name) in holidays.iter().take(10) {
        println!("{} | {}", date, name);
    }

    Ok(())
}
