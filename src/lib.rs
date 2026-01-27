//! no_std driver for the BH1750 / BH1750FVI ambient light sensor.
//!
//! `bh1750-embedded` is a small, `no_std` driver built on top of the
//! [`embedded-hal`](https://crates.io/crates/embedded-hal) traits.
//!
//! This crate is a clean reimplementation based on the existing Rust crate:
//! - `bh1750` by Jona Wilmsmann: <https://github.com/jona-wilmsmann/bh1750-rs>
//!
//! ## What this crate provides
//! - A blocking driver: [`Bh1750`]
//! - Resolution/mode selection via [`Resolution`]
//! - MTreg (measurement time) tuning via [`MeasurementTime`]
//! - Optional async API behind the `async` feature: [`r#async::Bh1750Async`]
//!
//! ## Blocking usage
//!
//! ```
//! use bh1750_embedded::{Address, Bh1750, Error, Resolution};
//!
//! fn example<I2C, D, E>(i2c: I2C, delay: D) -> Result<(), Error<E>>
//! where
//!     I2C: embedded_hal::i2c::I2c<Error = E>,
//!     D: embedded_hal::delay::DelayNs,
//!     E: embedded_hal::i2c::Error,
//! {
//!     let mut sensor = Bh1750::new(i2c, delay, Address::Low);
//!
//!     // One-shot measurement (driver waits long enough before reading):
//!     let _lux = sensor.one_time_measurement(Resolution::High)?;
//!     Ok(())
//! }
//! ```
//!
//! ## Continuous mode
//!
//! ```
//! use bh1750_embedded::{Address, Bh1750, Error, Resolution};
//!
//! fn example<I2C, D, E>(i2c: I2C, delay: D) -> Result<(), Error<E>>
//! where
//!     I2C: embedded_hal::i2c::I2c<Error = E>,
//!     D: embedded_hal::delay::DelayNs,
//!     E: embedded_hal::i2c::Error,
//! {
//!     let mut sensor = Bh1750::new(i2c, delay, Address::Low);
//!     sensor.start_continuous_measurement(Resolution::High)?;
//!
//!     // Wait approximately the measurement time before reading.
//!     let _ms = sensor.typical_measurement_time_ms(Resolution::High);
//!     let _lux = sensor.current_measurement_lux(Resolution::High)?;
//!     Ok(())
//! }
//! ```
//!
//! ## Async usage
//!
//! Enable the `async` feature to use the async driver API in [`r#async`].
//!
//! ```
//! # fn main() {}
//! #
//! # #[cfg(feature = "async")]
//! # mod async_example {
//! use bh1750_embedded::{Address, Error, Resolution};
//! use bh1750_embedded::r#async::Bh1750Async;
//!
//! pub async fn example<I2C, D, E>(i2c: I2C, delay: D) -> Result<(), Error<E>>
//! where
//!     I2C: embedded_hal_async::i2c::I2c<Error = E>,
//!     D: embedded_hal_async::delay::DelayNs,
//!     E: embedded_hal::i2c::Error,
//! {
//!     let mut sensor = Bh1750Async::new(i2c, delay, Address::Low);
//!     let _lux = sensor.one_time_measurement(Resolution::High).await?;
//!     Ok(())
//! }
//! # }
//! ```

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
