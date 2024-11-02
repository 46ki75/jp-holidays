use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Response {
    pub name: String,
    pub date: String,
    pub public: bool,
    pub holiday: bool,
}

impl From<crate::holiday::Holiday> for Response {
    fn from(holiday: crate::holiday::Holiday) -> Self {
        Self {
            name: holiday.name,
            date: holiday.date.to_string(),
            public: true,
            holiday: true,
        }
    }
}
