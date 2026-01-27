//! BH1750 / BH1750FVI ambient light sensor driver.
//!
//! A small, `no_std` driver built on the [`embedded-hal`](https://crates.io/crates/embedded-hal)
//! traits.
//!
//! Enable the `async` feature for the async driver in [`r#async`].

#![no_std]
#![deny(unsafe_code)]
#![deny(missing_docs)]

#[cfg(test)]
extern crate std;

mod error;
mod types;

mod bh1750;
pub use bh1750::Bh1750;

pub use error::Error;
pub use types::{Address, MeasurementTime, Resolution};

#[cfg(feature = "async")]
pub mod r#async;
