use embedded_hal_mock::eh1::i2c::{Mock as I2cMock, Transaction as I2cTransaction};

use bh1750_embedded::{Address, Bh1750, Resolution};

#[derive(Default)]
struct NoDelay;

impl embedded_hal::delay::DelayNs for NoDelay {
    fn delay_ns(&mut self, _ns: u32) {}

    fn delay_us(&mut self, _us: u32) {}

    fn delay_ms(&mut self, _ms: u32) {}
}

#[test]
fn one_time_measurement_writes_mode_then_reads_two_bytes() {
    // Resolution::High => 0x20
    let expectations = [
        I2cTransaction::write(0x23, vec![0x20]),
        I2cTransaction::read(0x23, vec![0x01, 0x20]),
    ];

    let i2c = I2cMock::new(&expectations);
    let mut sensor = Bh1750::new(i2c, NoDelay, Address::Low);

    let _lux = sensor.one_time_measurement(Resolution::High).unwrap();

    sensor.destroy().done();
}
