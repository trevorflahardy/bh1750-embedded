# bh1750-embedded

A platform-agnostic, `no_std` driver for the **BH1750 / BH1750FVI** ambient light sensor, built on the [`embedded-hal`](https://crates.io/crates/embedded-hal) traits.

This implementation is a clean reimplementation based on the existing Rust crate:
- `bh1750` by Jona Wilmsmann: <https://github.com/jona-wilmsmann/bh1750-rs>

## Features

- Blocking (synchronous) API using `embedded-hal` 1.0 (`I2c` + `DelayNs`)
- Optional async I2C + delay API behind the `async` feature using `embedded-hal-async`

### Feature flags

- `async`: enable async driver API (`embedded-hal-async`)

## Blocking example

```rust,ignore
use bh1750_embedded::{Bh1750, Resolution};

// let i2c = todo!("provide an embedded-hal I2C implementation");
// let delay = todo!("provide an embedded-hal DelayNs implementation");

let mut sensor = Bh1750::new(i2c, delay, bh1750_embedded::Address::Low);

let lux = sensor.one_time_measurement(Resolution::High)?;
# Ok::<(), bh1750_embedded::Error<_>>(())
```

## Async example

Enable the feature:

```toml
[dependencies]
bh1750-embedded = { version = "0.1", features = ["async"] }
```

```rust,ignore
use bh1750_embedded::r#async::Bh1750Async;
use bh1750_embedded::Resolution;

// let i2c = todo!("provide an embedded-hal-async I2C implementation");
// let delay = todo!("provide an embedded-hal-async DelayNs implementation");

let mut sensor = Bh1750Async::new(i2c, delay, bh1750_embedded::Address::Low);

let lux = sensor.one_time_measurement(Resolution::High).await?;
# Ok::<(), bh1750_embedded::Error<_>>(())
```

## License

MIT
