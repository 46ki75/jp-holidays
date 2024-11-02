use chrono::Datelike;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Response {
    pub name: String,
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
            name: holiday.name,
            date: holiday.date.to_string(),
            year: holiday.date.year(),
            month: holiday.date.month(),
            day: holiday.date.day(),
            public: true,
            holiday: true,
        }
    }
}
