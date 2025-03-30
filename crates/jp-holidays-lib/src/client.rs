pub struct Client {
    data: std::collections::BTreeMap<chrono::NaiveDate, String>,
}

impl Client {
    pub async fn init() -> Result<Self, crate::error::Error> {
        let holiday_repository = std::sync::Arc::new(crate::repository::HolidayRepositoryImpl);

        let holiday_service =
            std::sync::Arc::new(crate::service::HolidayService { holiday_repository });

        let csv = holiday_service.get_utf8_csv_string().await?;

        let data = holiday_service.deserialize_csv(&csv)?;

        Ok(Self { data })
    }

    #[cfg(test)]
    async fn init_stub() -> Result<Self, crate::error::Error> {
        let holiday_repository = std::sync::Arc::new(crate::repository::HolidayRepositoryStub);

        let holiday_service =
            std::sync::Arc::new(crate::service::HolidayService { holiday_repository });

        let csv = holiday_service.get_utf8_csv_string().await?;

        let data = holiday_service.deserialize_csv(&csv)?;

        Ok(Self { data })
    }

    pub fn get_holiday(
        &self,
        year: i32,
        month: u32,
        day: u32,
    ) -> Result<Option<&str>, crate::error::Error> {
        let native_date = chrono::NaiveDate::from_ymd_opt(year, month, day).ok_or(
            crate::error::Error::InvalidDate(format!(
                "不正な日付です: {}年 {}月 {}日",
                year, month, day
            )),
        )?;

        Ok(self.data.get(&native_date).map(|s| s.as_str()))
    }

    pub fn is_holiday(&self, year: i32, month: u32, day: u32) -> Result<bool, crate::error::Error> {
        let native_date = chrono::NaiveDate::from_ymd_opt(year, month, day).ok_or(
            crate::error::Error::InvalidDate(format!(
                "不正な日付です: {}年 {}月 {}日",
                year, month, day
            )),
        )?;

        let holiday = self.data.get(&native_date);

        Ok(holiday.is_some())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_holiday_known_date() {
        let client = Client::init_stub().await.unwrap();
        let holiday = client.get_holiday(1955, 1, 1).unwrap();
        assert_eq!(holiday, Some("元日"));
    }

    #[tokio::test]
    async fn test_get_holiday_unknown_date() {
        let client = Client::init_stub().await.unwrap();
        let holiday = client.get_holiday(1955, 1, 2).unwrap();
        assert_eq!(holiday, None);
    }

    #[tokio::test]
    async fn test_is_holiday_true() {
        let client = Client::init_stub().await.unwrap();
        let is_holiday = client.is_holiday(1955, 5, 5).unwrap();
        assert!(is_holiday);
    }

    #[tokio::test]
    async fn test_is_holiday_false() {
        let client = Client::init_stub().await.unwrap();
        let is_holiday = client.is_holiday(1955, 5, 4).unwrap();
        assert!(!is_holiday);
    }

    #[tokio::test]
    async fn test_invalid_date() {
        let client = Client::init_stub().await.unwrap();
        let result = client.get_holiday(1955, 2, 30);
        assert!(result.is_err());
    }
}
