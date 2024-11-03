mod date_format;
mod error;
mod holiday;
mod response;
mod util;

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

    let responses: Vec<response::Response> = holidays.into_iter().map(Into::into).collect();

    for response in responses {
        println!("{:?}", response);
    }

    let mut current_date = first_date;

    while current_date <= last_date {
        println!("{:?}", current_date);
        current_date = current_date
            .checked_add_signed(chrono::Duration::days(1))
            .ok_or(error::Error::InvalidDate(
                "Failed to add a day.".to_string(),
            ))?;
    }

    println!("{} ~ {}", first_date, last_date);

    Ok(())
}
