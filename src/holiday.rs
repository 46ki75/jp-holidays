use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Holiday {
    #[serde(rename = "国民の祝日・休日月日")]
    #[serde(with = "crate::date_format")]
    date: chrono::NaiveDate,

    #[serde(rename = "国民の祝日・休日名称")]
    name: String,
}

impl Holiday {
    pub fn deserialize_from_csv(csv: &str) -> Result<Vec<Holiday>, crate::error::Error> {
        let mut reader = csv::Reader::from_reader(csv.as_bytes());
        reader
            .deserialize()
            .map(|result| result.map_err(crate::error::Error::from))
            .collect()
    }
}
