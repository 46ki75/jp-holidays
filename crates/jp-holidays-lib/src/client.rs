use chrono::{Datelike, NaiveDate};

pub struct Client {
    data: std::collections::BTreeMap<NaiveDate, String>,
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

    pub fn get_holiday(&self, date: NaiveDate) -> Option<&str> {
        self.data.get(&date).map(|s| s.as_str())
    }

    pub fn get_holiday_ymd(
        &self,
        year: i32,
        month: u32,
        day: u32,
    ) -> Result<Option<&str>, crate::error::Error> {
        let date =
            NaiveDate::from_ymd_opt(year, month, day).ok_or(crate::error::Error::InvalidDate(
                format!("不正な日付です: {}年 {}月 {}日", year, month, day),
            ))?;
        Ok(self.get_holiday(date))
    }

    pub fn is_holiday(&self, date: NaiveDate) -> bool {
        self.data.contains_key(&date)
    }

    pub fn is_holiday_ymd(
        &self,
        year: i32,
        month: u32,
        day: u32,
    ) -> Result<bool, crate::error::Error> {
        let date =
            NaiveDate::from_ymd_opt(year, month, day).ok_or(crate::error::Error::InvalidDate(
                format!("不正な日付です: {}年 {}月 {}日", year, month, day),
            ))?;
        Ok(self.is_holiday(date))
    }

    pub fn is_day_off(&self, date: NaiveDate) -> bool {
        matches!(date.weekday(), chrono::Weekday::Sat | chrono::Weekday::Sun)
            || self.is_holiday(date)
    }

    pub fn is_day_off_ymd(
        &self,
        year: i32,
        month: u32,
        day: u32,
    ) -> Result<bool, crate::error::Error> {
        let date =
            NaiveDate::from_ymd_opt(year, month, day).ok_or(crate::error::Error::InvalidDate(
                format!("不正な日付です: {}年 {}月 {}日", year, month, day),
            ))?;
        Ok(self.is_day_off(date))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_holiday_known_date() {
        let client = Client::init_stub().await.unwrap();
        let holiday = client.get_holiday_ymd(1955, 1, 1).unwrap();
        assert_eq!(holiday, Some("元日"));
    }

    #[tokio::test]
    async fn test_get_holiday_unknown_date() {
        let client = Client::init_stub().await.unwrap();
        let holiday = client.get_holiday_ymd(1955, 1, 2).unwrap();
        assert_eq!(holiday, None);
    }

    #[tokio::test]
    async fn test_is_holiday_true() {
        let client = Client::init_stub().await.unwrap();
        let is_holiday = client.is_holiday_ymd(1955, 5, 5).unwrap();
        assert!(is_holiday);
    }

    #[tokio::test]
    async fn test_is_holiday_false() {
        let client = Client::init_stub().await.unwrap();
        let is_holiday = client.is_holiday_ymd(1955, 5, 4).unwrap();
        assert!(!is_holiday);
    }

    #[tokio::test]
    async fn test_invalid_date() {
        let client = Client::init_stub().await.unwrap();
        let result = client.get_holiday_ymd(1955, 2, 30);
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_is_day_off_holiday() {
        let client = Client::init_stub().await.unwrap();
        let is_day_off = client.is_day_off_ymd(1955, 1, 1).unwrap();
        assert!(is_day_off);
    }

    #[tokio::test]
    async fn test_is_day_off_weekend() {
        let client = Client::init_stub().await.unwrap();
        let is_day_off = client.is_day_off_ymd(1955, 1, 8).unwrap();
        assert!(is_day_off);
    }

    #[tokio::test]
    async fn test_is_day_off_weekday_non_holiday() {
        let client = Client::init_stub().await.unwrap();
        let is_day_off = client.is_day_off_ymd(1955, 1, 5).unwrap();
        assert!(!is_day_off);
    }
}
