//! Public configuration types.

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
