use chrono::NaiveDate;
use jp_holidays_lib::client::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::init().await?;

    // 祝日かどうか確認
    let date = NaiveDate::from_ymd_opt(1956, 3, 21).ok_or("存在しない日付です".to_string())?;

    let is_holiday = client.is_holiday(date);

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
