use jp_holidays_lib::client::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::init().await?;

    // 祝日を取得
    let date = time::Date::from_calendar_date(1955, time::Month::November, 23)
        .ok()
        .ok_or("存在しない日付です".to_string())?;

    let maybe_holiday = client.get_holiday(date);

    match maybe_holiday {
        Some(holiday) => println!("1955年 11月 23日 は{}", holiday),
        None => println!("1955年 11月 23日 は祝日ではありません"),
    };

    // 祝日かどうか確認
    let date = time::Date::from_calendar_date(1956, time::Month::March, 21)
        .ok()
        .ok_or("存在しない日付です".to_string())?;

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
