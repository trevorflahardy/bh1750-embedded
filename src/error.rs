//! Error types for the BH1750 driver.
//!
//! This module defines the [`Error`] type that can occur when interacting
//! with the BH1750 sensor. The error type is generic over the underlying
//! I2C error type, allowing it to work with any `embedded-hal` I2C
//! implementation.
//!
//! # Example: Error Handling
//!
//! ```
//! use bh1750_embedded::{Address, Bh1750, Error, MeasurementTime, Resolution};
//!
//! # fn example<I2C, D, E>(i2c: I2C, delay: D) -> Result<(), Error<E>>
//! # where
//! #     I2C: embedded_hal::i2c::I2c<Error = E>,
//! #     D: embedded_hal::delay::DelayNs,
//! #     E: embedded_hal::i2c::Error,
//! # {
//! let mut sensor = Bh1750::new(i2c, delay, Address::Low);
//!
//! match sensor.one_time_measurement(Resolution::High) {
//!     Ok(lux) => {
//!         // Use the lux value
//!         # let _ = lux;
//!     }
//!     Err(Error::I2c(_e)) => {
//!         // Handle I2C communication error
//!     }
//!     Err(Error::MeasurementTimeOutOfRange) => {
//!         // Handle invalid measurement time
//!     }
//! }
//! # Ok(())
//! # }
//! ```
//!
//! # Example: Validating Measurement Time
//!
//! ```
//! use bh1750_embedded::{Error, MeasurementTime};
//!
//! // This will succeed (value in range 31..=254)
//! let valid_mt = MeasurementTime::new(100);
//! assert!(valid_mt.is_some());
//!
//! // This will fail (value out of range)
//! let invalid_mt = MeasurementTime::new(10);
//! assert!(invalid_mt.is_none());
//! ```

use embedded_hal::i2c as ehal;

/// Driver error.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error<E> {
    /// Underlying I2C bus error.
    I2c(E),
    /// Measurement time register was outside the allowed range.
    MeasurementTimeOutOfRange,
}

impl<E> From<E> for Error<E> {
    fn from(e: E) -> Self {
        Self::I2c(e)
    }
}

impl<E> ehal::Error for Error<E>
where
    E: ehal::Error,
{
    fn kind(&self) -> ehal::ErrorKind {
        match self {
            Self::I2c(e) => e.kind(),
            Self::MeasurementTimeOutOfRange => ehal::ErrorKind::Other,
        }
    }
}
