//! Async API (requires the `async` feature).

use embedded_hal_async::delay::DelayNs;
use embedded_hal_async::i2c::I2c;

use crate::types::{Address, MeasurementTime, Resolution};
use crate::Error;

const POWER_DOWN: u8 = 0b0000_0000;
const POWER_ON: u8 = 0b0000_0001;
const RESET: u8 = 0b0000_0111;

const CHANGE_MTREG_HIGH: u8 = 0b0100_0000;
const CHANGE_MTREG_LOW: u8 = 0b0110_0000;

/// Async BH1750 / BH1750FVI driver.
#[derive(Debug)]
pub struct Bh1750Async<I2C, D> {
    i2c: I2C,
    delay: D,
    address: u8,
    mtreg: u8,
}

impl<I2C, D> Bh1750Async<I2C, D> {
    /// Create a new driver.
    #[must_use]
    pub fn new(i2c: I2C, delay: D, address: Address) -> Self {
        Self {
            i2c,
            delay,
            address: address.addr(),
            mtreg: MeasurementTime::DEFAULT.value(),
        }
    }

    /// Destroy the driver and return the underlying I2C bus.
    #[must_use]
    pub fn destroy(self) -> I2C {
        self.i2c
    }

    /// Get the current MTreg value.
    #[must_use]
    pub const fn measurement_time_raw(&self) -> u8 {
        self.mtreg
    }
}

impl<I2C, D, E> Bh1750Async<I2C, D>
where
    I2C: I2c<Error = E>,
    D: DelayNs,
{
    /// Power on.
    pub async fn power_on(&mut self) -> Result<(), Error<E>> {
        self.write_byte(POWER_ON).await
    }

    /// Power down.
    pub async fn power_down(&mut self) -> Result<(), Error<E>> {
        self.write_byte(POWER_DOWN).await
    }

    /// Reset data register.
    pub async fn reset(&mut self) -> Result<(), Error<E>> {
        self.write_byte(RESET).await
    }

    /// Set the measurement time register (MTreg).
    pub async fn set_measurement_time(&mut self, mt: MeasurementTime) -> Result<(), Error<E>> {
        let v = mt.value();

        self.write_byte(CHANGE_MTREG_HIGH | (v >> 5)).await?;
        self.write_byte(CHANGE_MTREG_LOW | (v & 0b0001_1111))
            .await?;

        self.mtreg = v;
        Ok(())
    }

    /// Start continuous measurements at the given resolution.
    pub async fn start_continuous_measurement(
        &mut self,
        resolution: Resolution,
    ) -> Result<(), Error<E>> {
        self.write_byte(resolution.continuous_cmd()).await
    }

    /// Read the current measurement in lux.
    pub async fn current_measurement_lux(
        &mut self,
        resolution: Resolution,
    ) -> Result<f32, Error<E>> {
        let raw = self.read_u16().await?;
        Ok(self.raw_to_lux(raw, resolution))
    }

    /// Perform a one-time measurement at the given resolution.
    pub async fn one_time_measurement(&mut self, resolution: Resolution) -> Result<f32, Error<E>> {
        self.write_byte(resolution.one_time_cmd()).await?;

        let ms = self.typical_measurement_time_ms(resolution);
        let safe = ms.saturating_mul(12) / 10;
        self.delay.delay_ms(safe).await;

        self.current_measurement_lux(resolution).await
    }

    /// Compute the typical measurement time in milliseconds for the current MTreg.
    #[must_use]
    pub fn typical_measurement_time_ms(&self, resolution: Resolution) -> u32 {
        let base = resolution.typical_delay_ms();
        if self.mtreg == MeasurementTime::DEFAULT.value() {
            base
        } else {
            base.saturating_mul(self.mtreg as u32) / (MeasurementTime::DEFAULT.value() as u32)
        }
    }

    fn raw_to_lux(&self, raw: u16, resolution: Resolution) -> f32 {
        let mt_factor = (MeasurementTime::DEFAULT.value() as f32) / (self.mtreg as f32);
        (raw as f32) / 1.2 * mt_factor / resolution.resolution_divisor()
    }

    async fn write_byte(&mut self, b: u8) -> Result<(), Error<E>> {
        self.i2c.write(self.address, &[b]).await.map_err(Error::I2c)
    }

    async fn read_u16(&mut self) -> Result<u16, Error<E>> {
        let mut buf = [0u8; 2];
        self.i2c
            .read(self.address, &mut buf)
            .await
            .map_err(Error::I2c)?;
        Ok(u16::from_be_bytes(buf))
    }
}
