use chrono::NaiveDate;
use jp_holidays_lib::client::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::init().await?;

    // 休日かどうか確認
    let date = NaiveDate::from_ymd_opt(1956, 3, 21).ok_or("存在しない日付です".to_string())?;

    let is_day_off = client.is_day_off(date);

    println!(
        "1956 3月 21日 は{}",
        if is_day_off {
            "休日です"
        } else {
            "休日ではありません"
        }
    );

    Ok(())
}
