fn main() {
    // #[cfg(all(feature = "time", feature = "chrono"))]
    // compile_error!("Features `time` and `chrono` cannot be enabled at the same time.");

    // #[cfg(not(any(feature = "time", feature = "chrono")))]
    // compile_error!("Either feature `time` or `chrono` must be enabled.");
}
