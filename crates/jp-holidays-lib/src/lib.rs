#![deny(missing_docs)]

//! # jp-holidays-lib
//!
//! 内閣府の公開する [「国民の祝日」について](https://www8.cao.go.jp/chosei/shukujitsu/gaiyou.html) より
//! 祝日判定機能を提供するクレートです。
//!
//! ## 使用方法
//!
//! 最初にクライアントを初期化します。この際、内閣府のオリジンに祝日のデータが格納された CSV を取得します。
//!
//! ```ignore
//! let client = jp_holidays_lib::client::Client::init().await?;
//! ```
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

pub mod client;
pub mod error;
pub(crate) mod repository;
pub(crate) mod service;
