use jp_holidays_lib::client::Client;
use std::ops::Bound::{Excluded, Included};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::init().await?;

    // 2018年 の祝日のみを取得します。
    let start = time::Date::from_calendar_date(2018, time::Month::January, 1)
        .ok()
        .ok_or("存在しない日付です".to_string())?;
    let end = time::Date::from_calendar_date(2019, time::Month::January, 1)
        .ok()
        .ok_or("存在しない日付です".to_string())?;

    // 公開されている祝日をすべて取得します。その後範囲を絞ります。
    let holidays_2018 = client
        .list_holidays()
        .range((Included(start), Excluded(end)));

    for (date, name) in holidays_2018 {
        println!("{} | {}", date, name);
    }

    Ok(())
}
