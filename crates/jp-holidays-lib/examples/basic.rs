use jp_holidays_lib::client::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::init().await?;

    // 祝日を取得
    let maybe_holiday = client.get_holiday_ymd(1955, 11, 23);
    match maybe_holiday? {
        Some(holiday) => println!("1955年 11月 23日 は{}", holiday),
        None => println!("1955年 11月 23日 は祝日ではありません"),
    };

    // 祝日かどうか確認
    let is_holiday = client.is_holiday_ymd(1956, 3, 21)?;
    println!(
        "1956 3月 21日 は{}",
        if is_holiday {
            "祝日です"
        } else {
            "祝日ではありません"
        }
    );

    Ok(())
}
