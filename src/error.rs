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
