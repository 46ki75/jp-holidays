//! # jp-holidays-lib
//!
//! 内閣府の公開する [「国民の祝日」について](https://www8.cao.go.jp/chosei/shukujitsu/gaiyou.html) より
//! 祝日判定機能を提供するクレートです。
//!
//! ## 使用方法
//!
//! 最初にクライアントを初期化します。この際、内閣府のオリジンに祝日のデータが格納された CSV を取得します。
//!
//! ```no_run
//! let client = jp_holidays_lib::client::Client::init().await?;
//! ``````
//!
//! その後は以下の例のように使用してください。
//! Rust の日付操作のデファクトスタンダードである chrono をベースに API が提供されています。
//!
//! ```
//! use chrono::NaiveDate;
//! use jp_holidays_lib::client::Client;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = Client::init().await?;
//!
//!     // 祝日を取得
//!     let date = NaiveDate::from_ymd_opt(1955, 11, 23).ok_or("存在しない日付です".to_string())?;
//!
//!     let maybe_holiday = client.get_holiday(date);
//!
//!     match maybe_holiday {
//!         Some(holiday) => println!("1955年 11月 23日 は{}", holiday),
//!         None => println!("1955年 11月 23日 は祝日ではありません"),
//!     };
//!
//!     // 祝日かどうか確認
//!     let date = NaiveDate::from_ymd_opt(1956, 3, 21).ok_or("存在しない日付です".to_string())?;
//!
//!     let is_holiday = client.is_holiday(date);
//!
//!     println!(
//!         "1956 3月 21日 は{}",
//!         if is_holiday {
//!             "祝日です"
//!         } else {
//!             "祝日ではありません"
//!         }
//!     );
//!
//!     Ok(())
//! }
//! ```
//!
//! `Client` が提供する関数は以下の通りです。
//!
//! ### 関連関数
//!
//! - `init()`: クライアントを初期化します。
//!
//! ### メソッド
//!
//! - `get_holiday()`: `chrono::NaiveDate` を渡して祝日を取得します。
//! - `get_holiday_ymd()`: 年月日を渡して祝日を取得します。
//! - `is_holiday()`:  `chrono::NaiveDate` を渡して祝日かどうかを判定します。
//! - `is_holiday_ymd()`:  年月日を渡して祝日かどうかを判定します。
//! - `is_day_off()`: `chrono::NaiveDate` を渡して休日かどうかを判定します。
//! - `is_day_off_ymd.()`: 年月日を渡して休日かどうかを判定します。
//! - `list_holidays()`: 公開されている祝日をすべて取得します (`BTreeMap<NaiveDate, String>`)
//!
//! ## キャッシュの利用
//!
//! 非同期ランタイムに `tokio` を使用している場合、以下のようにキャッシュを活用できます。
//! 祝日のデータが格納された CSV をキャッシュできます。
//!
//! ```
//! use chrono::NaiveDate;
//! use jp_holidays_lib::{client::Client, error::Error};
//!
//! // Client::init() は非同期に内閣府から祝日情報を取得するため、
//! // tokio::sync::OnceCell を使って初回のみ初期化し、その後はキャッシュを使用します。
//! static CACHE: tokio::sync::OnceCell<Client> = tokio::sync::OnceCell::const_new();
//!
//! // キャッシュされた Client インスタンスを取得します
//! async fn get_client() -> Result<&'static Client, Error> {
//!     CACHE.get_or_try_init(Client::init).await
//! }
//!
//! // 実行用の関数（main から呼び出し）
//! // スコープを抜けても Client はキャッシュされ続けます
//! async fn execute() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = get_client().await?;
//!
//!     // 祝日を取得
//!     let date = NaiveDate::from_ymd_opt(1955, 11, 23).ok_or("存在しない日付です".to_string())?;
//!
//!     let maybe_holiday = client.get_holiday(date);
//!
//!     match maybe_holiday {
//!         Some(holiday) => println!("1955年 11月 23日 は{}", holiday),
//!         None => println!("1955年 11月 23日 は祝日ではありません"),
//!     };
//!
//!     // 祝日かどうか確認
//!     let date = NaiveDate::from_ymd_opt(1956, 3, 21).ok_or("存在しない日付です".to_string())?;
//!
//!     let is_holiday = client.is_holiday(date);
//!
//!     println!(
//!         "1956 3月 21日 は{}",
//!         if is_holiday {
//!             "祝日です"
//!         } else {
//!             "祝日ではありません"
//!         }
//!     );
//!
//!     Ok(())
//! }
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     for i in 0..5 {
//!         let start = std::time::Instant::now();
//!         execute().await?;
//!         let duration = start.elapsed();
//!         println!("{}回目の実行時間: {:?}\n", i + 1, duration);
//!     }
//!
//!     Ok(())
//! }
//! ```

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
/// - `get_holiday_ymd()`: 年月日を渡して祝日を取得します。
/// - `is_holiday()`:  `chrono::NaiveDate` を渡して祝日かどうかを判定します。
/// - `is_holiday_ymd()`:  年月日を渡して祝日かどうかを判定します。
/// - `is_day_off()`: `chrono::NaiveDate` を渡して休日かどうかを判定します。
/// - `is_day_off_ymd.()`: 年月日を渡して休日かどうかを判定します。
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
    /// use chrono::NaiveDate;
    /// use jp_holidays_lib::client::Client;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::init().await?;
    ///
    ///     // 祝日を取得
    ///     let date = NaiveDate::from_ymd_opt(1955, 11, 23).ok_or("存在しない日付です".to_string())?;
    ///
    ///     let maybe_holiday = client.get_holiday(date);
    ///
    ///     match maybe_holiday {
    ///         Some(holiday) => println!("1955年 11月 23日 は{}", holiday),
    ///         None => println!("1955年 11月 23日 は祝日ではありません"),
    ///     };
    ///
    ///     // 祝日かどうか確認
    ///     let date = NaiveDate::from_ymd_opt(1956, 3, 21).ok_or("存在しない日付です".to_string())?;
    ///
    ///     let is_holiday = client.is_holiday(date);
    ///
    ///     println!(
    ///         "1956 3月 21日 は{}",
    ///         if is_holiday {
    ///             "祝日です"
    ///         } else {
    ///             "祝日ではありません"
    ///         }
    ///     );
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## キャッシュの利用
    ///
    /// 非同期ランタイムに `tokio` を使用している場合、以下のようにキャッシュを活用できます。
    ///
    /// ```
    /// use chrono::NaiveDate;
    /// use jp_holidays_lib::{client::Client, error::Error};
    ///
    /// // Client::init() は非同期に内閣府から祝日情報を取得するため、
    /// // tokio::sync::OnceCell を使って初回のみ初期化し、その後はキャッシュを使用します。
    /// static CACHE: tokio::sync::OnceCell<Client> = tokio::sync::OnceCell::const_new();
    ///
    /// // キャッシュされた Client インスタンスを取得します
    /// async fn get_client() -> Result<&'static Client, Error> {
    ///     CACHE.get_or_try_init(Client::init).await
    /// }
    ///
    /// // 実行用の関数（main から呼び出し）
    /// // スコープを抜けても Client はキャッシュされ続けます
    /// async fn execute() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = get_client().await?;
    ///
    ///     // 祝日を取得
    ///     let date = NaiveDate::from_ymd_opt(1955, 11, 23).ok_or("存在しない日付です".to_string())?;
    ///
    ///     let maybe_holiday = client.get_holiday(date);
    ///
    ///     match maybe_holiday {
    ///         Some(holiday) => println!("1955年 11月 23日 は{}", holiday),
    ///         None => println!("1955年 11月 23日 は祝日ではありません"),
    ///     };
    ///
    ///     // 祝日かどうか確認
    ///     let date = NaiveDate::from_ymd_opt(1956, 3, 21).ok_or("存在しない日付です".to_string())?;
    ///
    ///     let is_holiday = client.is_holiday(date);
    ///
    ///     println!(
    ///         "1956 3月 21日 は{}",
    ///         if is_holiday {
    ///             "祝日です"
    ///         } else {
    ///             "祝日ではありません"
    ///         }
    ///     );
    ///
    ///     Ok(())
    /// }
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     for i in 0..5 {
    ///         let start = std::time::Instant::now();
    ///         execute().await?;
    ///         let duration = start.elapsed();
    ///         println!("{}回目の実行時間: {:?}\n", i + 1, duration);
    ///     }
    ///
    ///     Ok(())
    /// }
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
    #[doc = include_str!("../../../README.md")]
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
    /// use chrono::NaiveDate;
    /// use jp_holidays_lib::client::Client;
    /// use std::ops::Bound::{Excluded, Included};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::init().await?;
    ///
    ///     // 2018年 の祝日のみを取得します。
    ///     let start = NaiveDate::from_ymd_opt(2018, 1, 1).ok_or("存在しない日付です".to_string())?;
    ///     let end = NaiveDate::from_ymd_opt(2019, 1, 1).ok_or("存在しない日付です".to_string())?;
    ///
    ///     // 公開されている祝日をすべて取得します。その後範囲を絞ります。
    ///     let holidays_2018 = client
    ///         .list_holidays()
    ///         .range((Included(start), Excluded(end)));
    ///
    ///     for (date, name) in holidays_2018 {
    ///         println!("{} | {}", date, name);
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn list_holidays(&self) -> &std::collections::BTreeMap<NaiveDate, String> {
        &self.data
    }

    ///　`chrono::NaiveDate` を渡して祝日を取得します。
    ///
    /// ## 使用例
    ///
    /// ```
    /// use chrono::NaiveDate;
    /// use jp_holidays_lib::client::Client;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::init().await?;
    ///
    ///     // 祝日を取得
    ///     let date = NaiveDate::from_ymd_opt(1955, 11, 23).ok_or("存在しない日付です".to_string())?;
    ///
    ///     let maybe_holiday = client.get_holiday(date);
    ///
    ///     match maybe_holiday {
    ///         Some(holiday) => println!("1955年 11月 23日 は{}", holiday),
    ///         None => println!("1955年 11月 23日 は祝日ではありません"),
    ///     };
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn get_holiday(&self, date: NaiveDate) -> Option<&str> {
        self.data.get(&date).map(|s| s.as_str())
    }

    ///　年月日を渡して祝日を取得します。
    ///
    /// ## 使用例
    /// ```
    /// use jp_holidays_lib::client::Client;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::init().await?;
    ///
    ///     // 祝日かどうか確認
    ///     let is_holiday = client.is_holiday_ymd(1956, 3, 21)?;
    ///     println!(
    ///         "1956 3月 21日 は{}",
    ///         if is_holiday {
    ///             "祝日です"
    ///         } else {
    ///             "祝日ではありません"
    ///         }
    ///     );
    ///
    ///     Ok(())
    /// }
    ///
    /// ```
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

    ///　`chrono::NaiveDate` を渡して祝日かどうか確認します。
    ///
    /// ## 使用例
    ///
    /// ```
    /// use chrono::NaiveDate;
    /// use jp_holidays_lib::client::Client;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::init().await?;
    ///
    ///     // 祝日かどうか確認
    ///     let date = NaiveDate::from_ymd_opt(1956, 3, 21).ok_or("存在しない日付です".to_string())?;
    ///
    ///     let is_holiday = client.is_holiday(date);
    ///
    ///     println!(
    ///         "1956 3月 21日 は{}",
    ///         if is_holiday {
    ///             "祝日です"
    ///         } else {
    ///             "祝日ではありません"
    ///         }
    ///     );
    ///
    ///     Ok(())
    /// }
    ///
    /// ```
    pub fn is_holiday(&self, date: NaiveDate) -> bool {
        self.data.contains_key(&date)
    }

    ///　年月日を渡して祝日かどうか確認します。
    ///
    /// ## 使用例
    ///
    /// ```
    /// use jp_holidays_lib::client::Client;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::init().await?;
    ///
    ///     // 祝日かどうか確認
    ///     let is_holiday = client.is_holiday_ymd(1956, 3, 21)?;
    ///     println!(
    ///         "1956 3月 21日 は{}",
    ///         if is_holiday {
    ///             "祝日です"
    ///         } else {
    ///             "祝日ではありません"
    ///         }
    ///     );
    ///
    ///     Ok(())
    /// }
    ///
    /// ```
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

    ///　`chrono::NaiveDate` を渡して**休日**(祝日+土日)かどうか確認します。
    ///
    /// ## 使用例
    ///
    /// ```
    /// use chrono::NaiveDate;
    /// use jp_holidays_lib::client::Client;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::init().await?;
    ///
    ///     // 休日かどうか確認
    ///     let date = NaiveDate::from_ymd_opt(1956, 3, 21).ok_or("存在しない日付です".to_string())?;
    ///
    ///     let is_day_off = client.is_day_off(date);
    ///
    ///     println!(
    ///         "1956 3月 21日 は{}",
    ///         if is_day_off {
    ///             "休日です"
    ///         } else {
    ///             "休日ではありません"
    ///         }
    ///     );
    ///
    ///     Ok(())
    /// }    ///
    /// ```
    pub fn is_day_off(&self, date: NaiveDate) -> bool {
        matches!(date.weekday(), chrono::Weekday::Sat | chrono::Weekday::Sun)
            || self.is_holiday(date)
    }

    ///　年月日を渡して**休日**(祝日+土日)かどうか確認します。
    ///
    /// ## 使用例
    ///
    /// ```
    /// use jp_holidays_lib::client::Client;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::init().await?;
    ///
    ///     // 休日かどうか確認
    ///     let is_day_off = client.is_holiday_ymd(1956, 3, 21)?;
    ///     println!(
    ///         "1956 3月 22日 は{}",
    ///         if is_day_off {
    ///             "休日です"
    ///         } else {
    ///             "休日ではありません"
    ///         }
    ///     );
    ///
    ///     Ok(())
    /// }
    /// ```
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
