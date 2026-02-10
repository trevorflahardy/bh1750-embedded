//! Public configuration types for the BH1750 driver.
//!
//! This module contains the core types used to configure and interact with
//! the BH1750 sensor:
//!
//! - [`Address`] - I2C address selection based on the ADDR pin
//! - [`Resolution`] - Measurement resolution and mode selection
//! - [`MeasurementTime`] - Sensitivity adjustment via the MTreg register
//!
//! # Address Configuration
//!
//! The BH1750 can have one of two I2C addresses depending on the ADDR pin state:
//!
//! ```
//! use bh1750_embedded::Address;
//!
//! // ADDR pin connected to GND (or floating)
//! let addr_low = Address::Low;  // 0x23
//!
//! // ADDR pin connected to VCC
//! let addr_high = Address::High; // 0x5C
//!
//! // Custom address (for unusual configurations)
//! let addr_custom = Address::Custom(0x24);
//! ```
//!
//! # Resolution Modes
//!
//! The sensor provides three resolution modes with different accuracy
//! and measurement times:
//!
//! ```
//! use bh1750_embedded::Resolution;
//!
//! // High resolution mode: 1 lx resolution, ~120ms measurement time
//! let high = Resolution::High;
//!
//! // High resolution mode 2: 0.5 lx resolution, ~120ms measurement time
//! let high2 = Resolution::High2;
//!
//! // Low resolution mode: 4 lx resolution, ~16ms measurement time (faster)
//! let low = Resolution::Low;
//! ```
//!
//! # Measurement Time (MTreg)
//!
//! The MTreg register (31..=254) adjusts the sensor's sensitivity and
//! measurement time. Higher values increase sensitivity for low-light
//! conditions, while lower values reduce sensitivity for bright conditions.
//!
//! ```
//! use bh1750_embedded::MeasurementTime;
//!
//! // Default value (69)
//! let default_mt = MeasurementTime::DEFAULT;
//!
//! // High sensitivity for low light (254)
//! let high_sensitivity = MeasurementTime::new(254).unwrap();
//!
//! // Low sensitivity for bright light (31)
//! let low_sensitivity = MeasurementTime::new(31).unwrap();
//!
//! // Values outside the range return None
//! assert!(MeasurementTime::new(10).is_none());
//! assert!(MeasurementTime::new(255).is_none());
//! ```
//!
//! # Complete Example
//!
//! ```
//! use bh1750_embedded::{Address, Bh1750, MeasurementTime, Resolution};
//!
//! # fn example<I2C, D, E>(i2c: I2C, delay: D) -> Result<(), bh1750_embedded::Error<E>>
//! # where
//! #     I2C: embedded_hal::i2c::I2c<Error = E>,
//! #     D: embedded_hal::delay::DelayNs,
//! #     E: embedded_hal::i2c::Error,
//! # {
//! let mut sensor = Bh1750::new(i2c, delay, Address::Low);
//!
//! // Configure for low-light conditions
//! sensor.set_measurement_time(MeasurementTime::new(200).unwrap())?;
//!
//! // Use high resolution for maximum accuracy
//! let lux = sensor.one_time_measurement(Resolution::High2)?;
//! # Ok(())
//! # }
//! ```

/// I2C address selection for the BH1750.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Address {
    /// Address pin low: `0x23`.
    Low,
    /// Address pin high: `0x5C`.
    High,
    /// A custom 7-bit I2C address.
    Custom(u8),
}

impl Address {
    /// Resolve into a 7-bit I2C address.
    #[must_use]
    pub const fn addr(self) -> u8 {
        match self {
            Self::Low => 0x23,
            Self::High => 0x5C,
            Self::Custom(a) => a,
        }
    }
}

/// Resolution / measurement mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Resolution {
    /// 1 lx resolution, typical measurement time ~120ms.
    High,
    /// 0.5 lx resolution, typical measurement time ~120ms.
    High2,
    /// 4 lx resolution, typical measurement time ~16ms.
    Low,
}

impl Resolution {
    pub(crate) const fn one_time_cmd(self) -> u8 {
        match self {
            Self::High => 0b0010_0000,
            Self::High2 => 0b0010_0001,
            Self::Low => 0b0010_0011,
        }
    }

    pub(crate) const fn continuous_cmd(self) -> u8 {
        match self {
            Self::High => 0b0001_0000,
            Self::High2 => 0b0001_0001,
            Self::Low => 0b0001_0011,
        }
    }

    pub(crate) const fn typical_delay_ms(self) -> u32 {
        match self {
            Self::High | Self::High2 => 120,
            Self::Low => 16,
        }
    }

    pub(crate) const fn resolution_divisor(self) -> f32 {
        match self {
            Self::High => 1.0,
            Self::High2 => 2.0,
            Self::Low => 1.0,
        }
    }
}

/// Measurement time register value.
///
/// Allowed range is 31..=254. Default is 69.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MeasurementTime(u8);

impl MeasurementTime {
    /// Default MTreg value (69).
    pub const DEFAULT: Self = Self(69);
    /// Minimum MTreg value (31).
    pub const MIN: u8 = 31;
    /// Maximum MTreg value (254).
    pub const MAX: u8 = 254;

    /// Create a new measurement time.
    pub const fn new(value: u8) -> Option<Self> {
        if value >= Self::MIN && value <= Self::MAX {
            Some(Self(value))
        } else {
            None
        }
    }

    /// Get the raw MTreg value.
    #[must_use]
    pub const fn value(self) -> u8 {
        self.0
    }
}

impl From<MeasurementTime> for u8 {
    fn from(mt: MeasurementTime) -> Self {
        mt.value()
    }
}

impl From<u8> for MeasurementTime {
    fn from(value: u8) -> Self {
        Self::new(value).expect("MeasurementTime value out of range")
    }
}
