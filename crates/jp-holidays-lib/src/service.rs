#![deny(missing_docs)]

pub(crate) struct HolidayService {
    pub(crate) holiday_repository:
        std::sync::Arc<dyn crate::repository::HolidayRepository + Send + Sync>,
}

impl HolidayService {
    pub(crate) async fn fetch_shiftjis_csv_bytes(
        &self,
    ) -> Result<bytes::Bytes, crate::error::Error> {
        self.holiday_repository.fetch_csv().await
    }

    pub(crate) async fn parse_csv(
        &self,
        shiftjis_bytes: bytes::Bytes,
    ) -> Result<String, crate::error::Error> {
        let (cow, _, _) = encoding_rs::SHIFT_JIS.decode(&shiftjis_bytes[..]);

        let result = cow.into_owned();

        Ok(result)
    }

    pub(crate) fn deserialize_csv(
        &self,
        csv: &str,
    ) -> Result<std::collections::BTreeMap<crate::Date, String>, crate::error::Error> {
        csv.lines()
            .skip(1)
            .filter(|line| !line.trim().is_empty())
            .map(|line| {
                let mut parts = line.trim().splitn(2, ',');
                let date_str = parts
                    .next()
                    .ok_or_else(|| crate::error::Error::Parse("no date".into()))?;
                let name_str = parts
                    .next()
                    .ok_or_else(|| crate::error::Error::Parse("no name".into()))?;

                cfg_if::cfg_if! {
                    if #[cfg(feature = "chrono")]{
                    let date = chrono::NaiveDate::parse_from_str(date_str.trim(), "%Y/%m/%d")
                        .map_err(|e| crate::error::Error::Parse(e.to_string()))?;
                    } else if #[cfg(feature = "time")] {
                        let format = time::macros::format_description!("[year]/[month padding:none]/[day padding:none]");
                        let date = time::Date::parse(date_str.trim(), &format)
                        .map_err(|e| crate::error::Error::Parse(e.to_string()))?;
                    }
                }

                Ok((date, name_str.trim().to_string()))
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_csv() -> Result<(), crate::error::Error> {
        let holiday_repository = std::sync::Arc::new(crate::repository::HolidayRepositoryImpl);

        let holiday_service = HolidayService { holiday_repository };

        let csv = r#"国民の祝日・休日月日,国民の祝日・休日名称
        1955/1/1,元日
        1955/1/15,成人の日
        1955/3/21,春分の日
        "#;

        let results = holiday_service.deserialize_csv(csv)?;

        cfg_if::cfg_if! {
            if #[cfg(feature = "chrono")]{
                const FMT: &str = "%Y/%m/%d";
                let expected = std::collections::BTreeMap::from([
                    (
                        chrono::NaiveDate::parse_from_str("1955/1/1", FMT).unwrap(),
                        "元日".to_string(),
                    ),
                    (
                        chrono::NaiveDate::parse_from_str("1955/1/15", FMT).unwrap(),
                        "成人の日".to_string(),
                    ),
                    (
                        chrono::NaiveDate::parse_from_str("1955/3/21", FMT).unwrap(),
                        "春分の日".to_string(),
                    ),
                ]);
            } else if #[cfg(feature = "time")]{
                let expected = std::collections::BTreeMap::from([
                    (
                        time::Date::from_calendar_date(1955, time::Month::January, 1).unwrap(),
                        "元日".to_string(),
                    ),
                    (
                        time::Date::from_calendar_date(1955, time::Month::January, 15).unwrap(),
                        "成人の日".to_string(),
                    ),
                    (
                        time::Date::from_calendar_date(1955, time::Month::March, 21).unwrap(),
                        "春分の日".to_string(),
                    ),
                ]);
            }
        }

        assert_eq!(results, expected);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_utf8_csv_string() -> Result<(), crate::error::Error> {
        let holiday_repository = std::sync::Arc::new(crate::repository::HolidayRepositoryStub);

        let holiday_service = HolidayService { holiday_repository };

        let shiftjis_bytes = holiday_service.fetch_shiftjis_csv_bytes().await?;

        let csv = holiday_service.parse_csv(shiftjis_bytes).await?;

        let _ = holiday_service.deserialize_csv(&csv)?;

        Ok(())
    }
}
