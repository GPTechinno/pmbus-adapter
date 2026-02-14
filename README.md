# pmbus-adapter

Async, `no_std` PMBus 1.4 protocol adapter built on top of
[`embedded-hal-async`](https://crates.io/crates/embedded-hal-async) I2C and
[`smbus-adapter`](https://crates.io/crates/smbus-adapter).

## Features

- **Full PMBus 1.4 command set** — typed methods for every standard command
  (voltage, current, temperature, fan, fault limits, status registers, etc.).
- **Data format codecs** — `Linear11`, `ULinear16`, and `DirectCoefficients`
  encode/decode helpers.
- **VOUT_MODE parsing** — decode and encode the `VOUT_MODE` register
  (ULinear16, VID, Direct, IEEE half).
- **Status bitflags** — strongly-typed `StatusByte`, `StatusWord`,
  `StatusVout`, `StatusIout`, and more.
- **`no_std` compatible** — zero heap allocations, suitable for bare-metal and
  RTOS targets.

## Usage

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
pmbus-adapter = "0.1"
```

```rust,no_run
use pmbus_adapter::{PmbusAdaptor, Linear11, VoutMode};
use smbus_adapter::SmbusAdaptor;

async fn example<B: embedded_hal_async::i2c::I2c>(bus: B) {
    let smbus = SmbusAdaptor::new(bus);
    let mut pmbus = PmbusAdaptor::new(smbus);

    let addr = 0x40;

    // Read output voltage (raw word), decode with VOUT_MODE exponent
    let mode = pmbus.get_vout_mode(addr).await.unwrap();
    let raw = pmbus.read_vout(addr).await.unwrap();

    // Read input current as LINEAR11
    let raw_iin = pmbus.read_iin(addr).await.unwrap();
    let iin = Linear11::from_raw(raw_iin).to_f32();
}
```

## Minimum Supported Rust Version

This crate requires **Rust 1.85.1** or later (edition 2024).

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
