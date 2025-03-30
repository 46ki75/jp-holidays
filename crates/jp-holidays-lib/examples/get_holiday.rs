use chrono::NaiveDate;
use jp_holidays_lib::client::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::init().await?;

    // 祝日を取得
    let date = NaiveDate::from_ymd_opt(1955, 11, 23).ok_or("存在しない日付です".to_string())?;

    let maybe_holiday = client.get_holiday(date);

    match maybe_holiday {
        Some(holiday) => println!("1955年 11月 23日 は{}", holiday),
        None => println!("1955年 11月 23日 は祝日ではありません"),
    };

    Ok(())
}
