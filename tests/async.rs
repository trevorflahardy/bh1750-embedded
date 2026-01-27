#![cfg(feature = "async")]

use core::cell::RefCell;

use bh1750_embedded::{r#async::Bh1750Async, Address, Resolution};

#[derive(Default)]
struct TestI2c {
    writes: RefCell<std::vec::Vec<(u8, std::vec::Vec<u8>)>>,
    reads: RefCell<std::vec::Vec<std::vec::Vec<u8>>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TestError;

impl embedded_hal::i2c::Error for TestError {
    fn kind(&self) -> embedded_hal::i2c::ErrorKind {
        embedded_hal::i2c::ErrorKind::Other
    }
}

impl embedded_hal_async::i2c::ErrorType for TestI2c {
    type Error = TestError;
}

impl embedded_hal_async::i2c::I2c for TestI2c {
    async fn read(&mut self, _address: u8, read: &mut [u8]) -> Result<(), Self::Error> {
        let mut q = self.reads.borrow_mut();
        let next = q.remove(0);
        read.copy_from_slice(&next);
        Ok(())
    }

    async fn write(&mut self, address: u8, write: &[u8]) -> Result<(), Self::Error> {
        self.writes.borrow_mut().push((address, write.to_vec()));
        Ok(())
    }

    async fn write_read(
        &mut self,
        _address: u8,
        _write: &[u8],
        _read: &mut [u8],
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    async fn transaction(
        &mut self,
        _address: u8,
        _operations: &mut [embedded_hal_async::i2c::Operation<'_>],
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[derive(Default)]
struct NoDelay;

impl embedded_hal_async::delay::DelayNs for NoDelay {
    async fn delay_ns(&mut self, _ns: u32) {}
}

#[tokio::test]
async fn async_one_time_measurement_writes_mode_then_reads_two_bytes() {
    let i2c = TestI2c::default();
    i2c.reads.replace(std::vec![std::vec![0x01, 0x20]]);

    let delay = NoDelay;
    let mut sensor = Bh1750Async::new(i2c, delay, Address::Low);

    let _lux = sensor.one_time_measurement(Resolution::High).await.unwrap();

    let writes = sensor.destroy().writes.into_inner();
    assert_eq!(writes[0], (0x23, std::vec![0x20]));
}
