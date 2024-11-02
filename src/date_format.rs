use chrono::NaiveDate;
use serde::{self, Deserialize, Deserializer};

const FORMAT: &str = "%Y/%m/%d";

pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    NaiveDate::parse_from_str(&s, FORMAT).map_err(|e| serde::de::Error::custom(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::de::value::{Error as DeError, StrDeserializer};
    use serde::de::IntoDeserializer;

    #[test]
    fn test_deserialize_valid_date() {
        let date_str: StrDeserializer<DeError> = "1955/05/03".into_deserializer();

        let result = deserialize(date_str);
        assert!(result.is_ok());

        let expected_date = NaiveDate::from_ymd_opt(1955, 5, 3).unwrap();
        assert_eq!(result.unwrap(), expected_date);
    }

    #[test]
    fn test_deserialize_invalid_date_format() {
        let date_str: StrDeserializer<DeError> = "1955-05-03".into_deserializer();

        let result = deserialize(date_str);
        assert!(result.is_err());
    }

    #[test]
    fn test_deserialize_non_date_string() {
        let date_str: StrDeserializer<DeError> = "not a date".into_deserializer();

        let result = deserialize(date_str);
        assert!(result.is_err());
    }

    #[test]
    fn test_deserialize_empty_string() {
        let date_str: StrDeserializer<DeError> = "".into_deserializer();

        let result = deserialize(date_str);
        assert!(result.is_err());
    }

    #[test]
    fn test_deserialize_leap_year_date() {
        let date_str: StrDeserializer<DeError> = "2020/02/29".into_deserializer();

        let result = deserialize(date_str);
        assert!(result.is_ok());

        let expected_date = NaiveDate::from_ymd_opt(2020, 2, 29).unwrap();
        assert_eq!(result.unwrap(), expected_date);
    }

    #[test]
    fn test_deserialize_non_leap_year_date() {
        let date_str: StrDeserializer<DeError> = "2019/02/29".into_deserializer();

        let result = deserialize(date_str);
        assert!(result.is_err());
    }
}
