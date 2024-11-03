use chrono::Datelike;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Response {
    pub name: Option<String>,
    pub date: String,
    pub year: i32,
    pub month: u32,
    pub day: u32,
    pub day_of_week: String,
    pub day_of_week_ja: String,
    pub public: bool,
    pub holiday: bool,
}

impl From<crate::holiday::Holiday> for Response {
    fn from(holiday: crate::holiday::Holiday) -> Self {
        Self {
            name: Some(holiday.name),
            date: holiday.date.to_string(),
            year: holiday.date.year(),
            month: holiday.date.month(),
            day: holiday.date.day(),
            day_of_week: holiday.date.format("%A").to_string(),
            day_of_week_ja: Response::to_japanese_weekday(holiday.date.weekday()).to_string(),
            public: true,
            holiday: true,
        }
    }
}

impl From<chrono::NaiveDate> for Response {
    fn from(date: chrono::NaiveDate) -> Self {
        let holiday = matches!(date.weekday(), chrono::Weekday::Sat | chrono::Weekday::Sun);

        Self {
            name: None,
            date: date.to_string(),
            year: date.year(),
            month: date.month(),
            day: date.day(),
            day_of_week: date.format("%A").to_string(),
            day_of_week_ja: Response::to_japanese_weekday(date.weekday()).to_string(),
            public: false,
            holiday,
        }
    }
}

impl Response {
    /// path must end with a slash
    /// e.g. "./dist/api/v1/"
    pub fn save(&self, path: &str) -> Result<(), crate::error::Error> {
        let file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(format!("{}{}.json", path, self.date))?;
        serde_json::to_writer(file, self)?;
        Ok(())
    }

    pub fn to_japanese_weekday(weekday: chrono::Weekday) -> &'static str {
        match weekday {
            chrono::Weekday::Mon => "月",
            chrono::Weekday::Tue => "火",
            chrono::Weekday::Wed => "水",
            chrono::Weekday::Thu => "木",
            chrono::Weekday::Fri => "金",
            chrono::Weekday::Sat => "土",
            chrono::Weekday::Sun => "日",
        }
    }
}
