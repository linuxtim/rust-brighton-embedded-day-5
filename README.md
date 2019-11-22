# Rust Brighton RGB WS2812 Blinky (Week 5)

A basic demo using the an ST [Nucleo-F411RE](https://www.st.com/en/evaluation-tools/nucleo-f411re.html) board with some WS2812 addressible RGB LEDs using the [Smart-leds](https://crates.io/crates/smart-leds) crate.

The code was adapted from one of the [smart-leds-samples](https://github.com/smart-leds-rs/smart-leds-samples), and uses the [stm32f4xx-hal](https://crates.io/crates/stm32f4xx-hal).

We use one of the stm32f4 SPI peripherals to generate the data signal for the LED string.  You will need to use external level shifting as appropriate.

- Make any necessary changes to suit your development environment (e.g. `openocd.gdb` `.cargo/config` etc.).
- Plug in the hardware
- Launch `openocd`
- `cargo run --release`

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
