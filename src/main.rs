mod date_format;
mod error;
mod holiday;
mod response;
mod util;

use chrono::NaiveDate;
use std::collections::HashSet;

#[tokio::main]
async fn main() -> Result<(), error::Error> {
    let raw_csv_bytes =
        util::fetch_csv("https://www8.cao.go.jp/chosei/shukujitsu/syukujitsu.csv").await?;

    let csv = util::to_utf8(&raw_csv_bytes)?;

    let holidays = holiday::Holiday::deserialize_from_csv(&csv)?;

    let first_date = holiday::Holiday::get_first_date(holidays.clone()).ok_or(
        error::Error::InvalidDate("Failed to get the first date.".to_string()),
    )?;

    let last_date = holiday::Holiday::get_last_date(holidays.clone()).ok_or(
        error::Error::InvalidDate("Failed to get the last date.".to_string()),
    )?;

    // let mut responses: Vec<response::Response>;

    let holiday_dates: HashSet<NaiveDate> = holidays.iter().map(|holiday| holiday.date).collect();

    let mut current_date = first_date;

    while current_date <= last_date {
        if holiday_dates.contains(&current_date) {
            println!("Holiday: {:?}", current_date);
        } else {
            println!("Not a holiday: {:?}", current_date);
        }

        current_date = current_date
            .checked_add_signed(chrono::Duration::days(1))
            .ok_or(error::Error::InvalidDate(
                "Failed to add a day.".to_string(),
            ))?;
    }

    println!("{} ~ {}", first_date, last_date);

    Ok(())
}
