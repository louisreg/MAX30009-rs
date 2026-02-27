//! Logging abstraction layer.
//!
//! - `defmt` backend when the `defmt` feature is enabled.
//! - `std` printing backend for tests.
//! - silent no-op backend for `no_std` production builds.
// =======================
// defmt backend
// =======================

#[cfg(feature = "defmt")]
pub use defmt::{debug, error, info, trace, warn};

// =======================
// std backend (tests only)
// =======================

#[cfg(all(not(feature = "defmt"), test))]
#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => { std::println!($($arg)*); };
}

#[cfg(all(not(feature = "defmt"), test))]
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => { std::println!($($arg)*); };
}

#[cfg(all(not(feature = "defmt"), test))]
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => { std::println!($($arg)*); };
}

#[cfg(all(not(feature = "defmt"), test))]
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => { std::eprintln!($($arg)*); };
}

#[cfg(all(not(feature = "defmt"), test))]
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => { std::eprintln!($($arg)*); };
}

// =======================
// no_std silent backend
// =======================

#[cfg(all(not(feature = "defmt"), not(test)))]
#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {};
}

#[cfg(all(not(feature = "defmt"), not(test)))]
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {};
}

#[cfg(all(not(feature = "defmt"), not(test)))]
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {};
}

#[cfg(all(not(feature = "defmt"), not(test)))]
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {};
}

#[cfg(all(not(feature = "defmt"), not(test)))]
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {};
}
