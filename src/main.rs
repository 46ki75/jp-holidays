mod date_format;
mod error;
mod holiday;
mod response;
mod response_list;
mod util;

use chrono::{Datelike, NaiveDate};
use std::collections::{HashMap, HashSet};

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
    let mut monthly_responses: HashMap<String, Vec<response::Response>> = HashMap::new();
    let mut yearly_responses: HashMap<String, Vec<response::Response>> = HashMap::new();

    // Initialize all months and years
    let mut current_date = first_date;
    while current_date <= last_date {
        let month_key = format!("{}-{:02}", current_date.year(), current_date.month());
        monthly_responses.entry(month_key).or_default();

        let year_key = format!("{}", current_date.year());
        yearly_responses.entry(year_key).or_default();

        // Move to the next month
        current_date = current_date
            .with_day(1)
            .unwrap()
            .checked_add_signed(chrono::Duration::days(32)) // Jump to the first day of the next month
            .unwrap()
            .with_day(1)
            .unwrap()
            .checked_sub_signed(chrono::Duration::days(1))
            .unwrap()
            .checked_add_signed(chrono::Duration::days(1))
            .unwrap();
    }

    // Process each day
    current_date = first_date;
    while current_date <= last_date {
        let response = if holiday_dates.contains(&current_date) {
            let holiday_response = response::Response::from(
                holidays
                    .iter()
                    .find(|holiday| holiday.date == current_date)
                    .ok_or(error::Error::InvalidDate(
                        "Failed to find a holiday.".to_string(),
                    ))?
                    .clone(),
            );

            let month_key = format!("{}-{:02}", current_date.year(), current_date.month());
            monthly_responses
                .entry(month_key)
                .or_default()
                .push(holiday_response.clone());

            let year_key = format!("{}", current_date.year());
            yearly_responses
                .entry(year_key)
                .or_default()
                .push(holiday_response.clone());

            holiday_response
        } else {
            response::Response::from(current_date)
        };

        // Save daily JSON file
        response.save(ROOT_DIR)?;

        current_date = current_date
            .checked_add_signed(chrono::Duration::days(1))
            .ok_or(error::Error::InvalidDate(
                "Failed to add a day.".to_string(),
            ))?;
    }

    // Save monthly response lists using ResponseList
    for (month, responses) in monthly_responses {
        let response_list = response_list::ResponseList { results: responses };
        response_list.save(ROOT_DIR, &month)?;
    }

    // Save yearly response lists using ResponseList
    for (year, responses) in yearly_responses {
        let response_list = response_list::ResponseList { results: responses };
        response_list.save(ROOT_DIR, &year)?;
    }

    println!("{} ~ {}", first_date, last_date);

    // Save the overall list as well
    let response_list = response_list::ResponseList::from(holidays);
    response_list.save(ROOT_DIR, "list")?;

    Ok(())
}
