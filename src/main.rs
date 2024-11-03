mod date_format;
mod error;
mod holiday;
mod response;
mod response_list;
mod util;

use chrono::NaiveDate;
use std::collections::HashSet;

#[tokio::main]
async fn main() -> Result<(), error::Error> {
    const ROOT_DIR: &str = "./dist/api/v1/";

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

    let is_dir_exists = std::path::Path::new(ROOT_DIR).exists();

    if is_dir_exists {
        std::fs::remove_dir_all(ROOT_DIR)?;
    }

    std::fs::create_dir_all(ROOT_DIR)?;

    let holiday_dates: HashSet<NaiveDate> = holidays.iter().map(|holiday| holiday.date).collect();

    let mut current_date = first_date;

    while current_date <= last_date {
        if holiday_dates.contains(&current_date) {
            response::Response::from(
                holidays
                    .iter()
                    .find(|holiday| holiday.date == current_date)
                    .ok_or(error::Error::InvalidDate(
                        "Failed to find a holiday.".to_string(),
                    ))?
                    .clone(),
            )
            .save(ROOT_DIR)?;
        } else {
            response::Response::from(current_date).save(ROOT_DIR)?;
        }

        current_date = current_date
            .checked_add_signed(chrono::Duration::days(1))
            .ok_or(error::Error::InvalidDate(
                "Failed to add a day.".to_string(),
            ))?;
    }

    println!("{} ~ {}", first_date, last_date);

    let response_list = response_list::ResponseList::from(holidays);

    response_list.save(ROOT_DIR)?;

    Ok(())
}
