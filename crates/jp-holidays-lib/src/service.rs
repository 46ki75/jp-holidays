pub(crate) struct HolidayService {
    pub(crate) holiday_repository:
        std::sync::Arc<dyn crate::repository::HolidayRepository + Send + Sync>,
}

impl HolidayService {
    pub(crate) async fn get_utf8_csv_string(&self) -> Result<String, crate::error::Error> {
        let sjis_csv_bytes = self.holiday_repository.fetch_csv().await?;

        let (cow, _, _) = encoding_rs::SHIFT_JIS.decode(&sjis_csv_bytes[..]);

        let result = cow.into_owned();

        Ok(result)
    }

    pub(crate) fn deserialize_csv(
        &self,
        csv: &str,
    ) -> Result<std::collections::BTreeMap<chrono::NaiveDate, String>, crate::error::Error> {
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

                let date = chrono::NaiveDate::parse_from_str(date_str.trim(), "%Y/%m/%d")
                    .map_err(|e| crate::error::Error::Parse(e.to_string()))?;

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

        assert_eq!(results, expected);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_utf8_csv_string() -> Result<(), crate::error::Error> {
        let holiday_repository = std::sync::Arc::new(crate::repository::HolidayRepositoryStub);

        let holiday_service = HolidayService { holiday_repository };

        let csv = holiday_service.get_utf8_csv_string().await?;

        let _ = holiday_service.deserialize_csv(&csv)?;

        Ok(())
    }
}
