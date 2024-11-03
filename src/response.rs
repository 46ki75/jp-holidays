use chrono::Datelike;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Response {
    pub name: Option<String>,
    pub date: String,
    pub year: i32,
    pub month: u32,
    pub day: u32,
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
            public: true,
            holiday: true,
        }
    }
}

impl From<chrono::NaiveDate> for Response {
    fn from(date: chrono::NaiveDate) -> Self {
        Self {
            name: None,
            date: date.to_string(),
            year: date.year(),
            month: date.month(),
            day: date.day(),
            public: false,
            holiday: false,
        }
    }
}
