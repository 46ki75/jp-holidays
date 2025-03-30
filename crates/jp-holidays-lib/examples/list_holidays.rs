use chrono::NaiveDate;
use jp_holidays_lib::client::Client;
use std::ops::Bound::{Excluded, Included};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::init().await?;

    // 2018年 の祝日のみを取得します。
    let start = NaiveDate::from_ymd_opt(2018, 1, 1).ok_or("存在しない日付です".to_string())?;
    let end = NaiveDate::from_ymd_opt(2019, 1, 1).ok_or("存在しない日付です".to_string())?;

    // 公開されている祝日をすべて取得します。その後範囲を絞ります。
    let holidays_2018 = client
        .list_holidays()
        .range((Included(start), Excluded(end)));

    for (date, name) in holidays_2018 {
        println!("{} | {}", date, name);
    }

    Ok(())
}
