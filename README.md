# Drogue IoT Grove SPI over UART

[![crates.io](https://img.shields.io/crates/v/drogue-grove-uart-spi.svg)](https://crates.io/crates/drogue-grove-uart-spi)
[![docs.rs](https://docs.rs/drogue-grove-uart-spi/badge.svg)](https://docs.rs/drogue-grove-uart-spi)
[![Matrix](https://img.shields.io/matrix/drogue-iot:matrix.org)](https://matrix.to/#/#drogue-iot:matrix.org)

This crate implements an SPI interface over UART, as used by
[Grove's LoRaWAN board](https://github.com/Seeed-Studio/Grove_LoRa_433MHz_and_915MHz_RF/blob/master/examples/Grove_LoRa_firmware/Grove_LoRa_firmware.ino).

## Usage

~~~rust
let serial = {
    Serial::usart6(
        p.USART6,
        (tx, rx),
        Config {
            baudrate: 57_600.bps(),
            ..Default::default()
        },
        clocks,
    )
    .unwrap()
};

let spi = UARTSPI::new(serial);

// use it with the LoRa driver

let lora = LoRa::new(spi, NoOpPin, NoOpPin, FREQUENCY, delay).unwrap();

// you can also get back the UART

let serial = spi.free();
~~~