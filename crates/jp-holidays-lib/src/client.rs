//! クレートのエントリーポイントである `Client` を定義しています。

use chrono::{Datelike, NaiveDate};

/// `jp_holidays_lib::client::Client::init()` にて初期化を行います。
///
/// ### 関連関数
///
/// - `init()`: クライアントを初期化します。
///
/// ### メソッド
///
/// - `get_holiday()`: `chrono::NaiveDate` を渡して祝日を取得します。
/// - `is_holiday()`:  `chrono::NaiveDate` を渡して祝日かどうかを判定します。
/// - `is_day_off()`: `chrono::NaiveDate` を渡して休日かどうかを判定します。
/// - `list_holidays()`: 公開されている祝日をすべて取得します (`BTreeMap<NaiveDate, String>`)
pub struct Client {
    data: std::collections::BTreeMap<NaiveDate, String>,
}

impl Client {
    /// クライアントを初期化します。
    ///
    /// ## 使用例
    ///
    /// ```
    #[doc = include_str!("../examples/basic.rs")]
    /// ```
    ///
    /// ## キャッシュの利用
    ///
    /// 非同期ランタイムに `tokio` を使用している場合、以下のようにキャッシュを活用できます。
    ///
    /// ```
    #[doc = include_str!("../examples/cache.rs")]
    /// ```    
    pub async fn init() -> Result<Self, crate::error::Error> {
        let holiday_repository = std::sync::Arc::new(crate::repository::HolidayRepositoryImpl);
        let holiday_service =
            std::sync::Arc::new(crate::service::HolidayService { holiday_repository });
        let shiftjis_bytes = holiday_service.fetch_shiftjis_csv_bytes().await?;
        let csv = holiday_service.parse_csv(shiftjis_bytes).await?;
        let data = holiday_service.deserialize_csv(&csv)?;
        Ok(Self { data })
    }

    #[cfg(test)]
    async fn init_stub() -> Result<Self, crate::error::Error> {
        let holiday_repository = std::sync::Arc::new(crate::repository::HolidayRepositoryStub);
        let holiday_service =
            std::sync::Arc::new(crate::service::HolidayService { holiday_repository });
        let shiftjis_bytes = holiday_service.fetch_shiftjis_csv_bytes().await?;
        let csv = holiday_service.parse_csv(shiftjis_bytes).await?;
        let data = holiday_service.deserialize_csv(&csv)?;
        Ok(Self { data })
    }

    /// 現在内閣府から公開されている範囲の祝日一覧を取得します。
    ///
    /// ## 使用例
    ///
    /// ```
    #[doc = include_str!("../examples/list_holidays.rs")]
    /// ```
    pub fn list_holidays(&self) -> &std::collections::BTreeMap<NaiveDate, String> {
        &self.data
    }

    ///　`chrono::NaiveDate` を渡して祝日を取得します。
    ///
    /// ## 使用例
    ///
    /// ```
    #[doc = include_str!("../examples/get_holiday.rs")]
    /// ```
    pub fn get_holiday(&self, date: NaiveDate) -> Option<&str> {
        self.data.get(&date).map(|s| s.as_str())
    }

    ///　`chrono::NaiveDate` を渡して祝日かどうか確認します。
    ///
    /// ## 使用例
    ///
    /// ```
    #[doc = include_str!("../examples/is_holiday.rs")]
    /// ```
    pub fn is_holiday(&self, date: NaiveDate) -> bool {
        self.data.contains_key(&date)
    }

    ///　`chrono::NaiveDate` を渡して**休日**(祝日+土日)かどうか確認します。
    ///
    /// ## 使用例
    ///
    /// ```
    #[doc = include_str!("../examples/is_day_off.rs")]
    /// ```
    pub fn is_day_off(&self, date: NaiveDate) -> bool {
        matches!(date.weekday(), chrono::Weekday::Sat | chrono::Weekday::Sun)
            || self.is_holiday(date)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[tokio::test]
    async fn test_get_holiday_known_date() {
        let client = Client::init_stub().await.unwrap();
        let date = NaiveDate::from_ymd_opt(1955, 1, 1).unwrap();
        let holiday = client.get_holiday(date).unwrap();
        assert_eq!(holiday, "元日");
    }

    #[tokio::test]
    async fn test_get_holiday_unknown_date() {
        let client = Client::init_stub().await.unwrap();
        let date = NaiveDate::from_ymd_opt(1955, 1, 2).unwrap();
        let holiday = client.get_holiday(date);
        assert_eq!(holiday, None);
    }

    #[tokio::test]
    async fn test_is_holiday_true() {
        let client = Client::init_stub().await.unwrap();
        let date = NaiveDate::from_ymd_opt(1955, 5, 5).unwrap();
        let is_holiday = client.is_holiday(date);
        assert!(is_holiday);
    }

    #[tokio::test]
    async fn test_is_holiday_false() {
        let client = Client::init_stub().await.unwrap();
        let date = NaiveDate::from_ymd_opt(1955, 5, 4).unwrap();
        let is_holiday = client.is_holiday(date);
        assert!(!is_holiday);
    }

    #[tokio::test]
    async fn test_is_day_off_holiday() {
        let client = Client::init_stub().await.unwrap();
        let date = NaiveDate::from_ymd_opt(1955, 1, 1).unwrap();
        let is_day_off = client.is_day_off(date);
        assert!(is_day_off);
    }

    #[tokio::test]
    async fn test_is_day_off_weekend() {
        let client = Client::init_stub().await.unwrap();
        let date = NaiveDate::from_ymd_opt(1955, 1, 8).unwrap();
        let is_day_off = client.is_day_off(date);
        assert!(is_day_off);
    }

    #[tokio::test]
    async fn test_is_day_off_weekday_non_holiday() {
        let client = Client::init_stub().await.unwrap();
        let date = NaiveDate::from_ymd_opt(1955, 1, 5).unwrap();
        let is_day_off = client.is_day_off(date);
        assert!(!is_day_off);
    }
}
