#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

pub mod client;
pub mod error;
pub(crate) mod repository;
pub(crate) mod service;

cfg_if::cfg_if! {
    if #[cfg(feature = "chrono")] {
        /// Alias of `chrono::NaiveDate`
        pub type Date = chrono::NaiveDate;
    } else if #[cfg(feature = "time")] {
        /// Alias of `time::Date`
        pub type Date = time::Date;
    }
}
