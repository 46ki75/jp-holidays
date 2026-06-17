#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

pub mod client;
pub mod error;

mod parse;

#[cfg(feature = "fetch")]
mod fetch;

pub use client::Client;
pub use error::Error;
