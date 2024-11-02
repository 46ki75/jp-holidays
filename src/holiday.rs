use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Holiday {
    #[serde(rename = "国民の祝日・休日月日")]
    #[serde(with = "crate::date_format")]
    pub date: chrono::NaiveDate,

    #[serde(rename = "国民の祝日・休日名称")]
    pub name: String,
}

impl Holiday {
    pub fn deserialize_from_csv(csv: &str) -> Result<Vec<Self>, crate::error::Error> {
        let mut reader = csv::Reader::from_reader(csv.as_bytes());
        reader
            .deserialize()
            .map(|result| result.map_err(crate::error::Error::from))
            .collect()
    }

    pub fn get_first_date(holidays: Vec<Self>) -> Option<chrono::NaiveDate> {
        holidays.iter().map(|holiday| holiday.date).min()
    }

    pub fn get_last_date(holidays: Vec<Self>) -> Option<chrono::NaiveDate> {
        holidays.iter().map(|holiday| holiday.date).max()
    }
}
