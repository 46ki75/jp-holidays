use jp_holidays_lib::client::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::init().await?;

    let holidays = client.list_holidays();

    for holiday in holidays.iter().take(10) {
        let (date, name) = holiday;
        println!("{}, {}", date, name);
    }

    Ok(())
}
