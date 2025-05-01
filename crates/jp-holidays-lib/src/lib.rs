#![deny(missing_docs)]

#![doc = include_str!("../README.md")]

pub mod client;
pub mod error;
pub(crate) mod repository;
pub(crate) mod service;
