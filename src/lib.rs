#![no_std]

//! SPI over UART
//!
//! The `UARTSPI` struct implements an SPI interface on top of a `UART`, using the protocol
//! used by Grove.
//!
//! Also see: https://github.com/Seeed-Studio/Grove_LoRa_433MHz_and_915MHz_RF/blob/master/examples/Grove_LoRa_firmware/Grove_LoRa_firmware.ino
//!
//! ## Protocol
//!
//! When writing, this is:
//!
//! ~~~
//! | 'W' | <reg> | <len> | <data>... |
//! ~~~
//!
//! There is no response after a write request has been processed.
//!
//! When reading, it is:
//!
//! ~~~
//! | 'R' | <reg> | <len> |
//! ~~~
//!
//! This will follow <len> bytes, which is the data read from SPI.
//!

use embedded_hal::blocking::spi::{Transfer, Write};
use embedded_hal::digital::v2::OutputPin;
use embedded_hal::serial;

/// SPI over UART
pub struct UARTSPI<UART> {
    uart: UART,
}

impl<UART> UARTSPI<UART> {
    /// Wrap the SPI protocol around the UART interface.
    pub fn new(uart: UART) -> Self {
        UARTSPI { uart }
    }

    /// Free the SPI interface and get back the UART
    pub fn free(self) -> UART {
        self.uart
    }
}

impl<UART, E> Transfer<u8> for UARTSPI<UART>
where
    UART: serial::Read<u8, Error = E> + serial::Write<u8, Error = E>,
{
    type Error = nb::Error<E>;

    fn transfer<'w>(&mut self, words: &'w mut [u8]) -> Result<&'w [u8], Self::Error> {
        let mut len = words.len();
        if len == 0 {
            return Ok(words);
        }

        // the buffer also contains the register as the first byte, but this is treated specially
        // on the protocol level
        len -= 1;

        // FIXME: handle with grace
        assert!(len <= 255);

        let reg = words[0];

        // write read request
        nb::block!(self.uart.write(b'R'))?;
        nb::block!(self.uart.write(reg))?;
        nb::block!(self.uart.write(len as u8))?;
        // flush
        nb::block!(self.uart.flush())?;

        #[cfg(feature = "dump")]
        log::info!("Sent R-request (reg: 0x{:02x}, len: {})", reg, len);

        // read len bytes

        for i in 0..len {
            words[i + 1] = nb::block!(self.uart.read())?;
            #[cfg(feature = "dump")]
            log::info!("Received: {:02x}", words[i + 1]);
        }

        Ok(words)
    }
}

impl<UART> Write<u8> for UARTSPI<UART>
where
    UART: serial::Write<u8>,
{
    type Error = nb::Error<UART::Error>;

    fn write(&mut self, words: &[u8]) -> Result<(), Self::Error> {
        let mut len = words.len();
        if len == 0 {
            return Ok(());
        }

        // the buffer also contains the register as the first byte, but this is treated specially
        // on the protocol level
        len -= 1;

        // FIXME: handle with grace
        assert!(len <= 255);

        let reg = words[0];
        nb::block!(self.uart.write(b'W'))?;
        nb::block!(self.uart.write(reg))?;
        nb::block!(self.uart.write(len as u8))?;
        for b in &words[1..] {
            nb::block!(self.uart.write(*b))?;
        }
        nb::block!(self.uart.flush())?;

        #[cfg(feature = "dump")]
        log::info!("Sent W-request (reg: 0x{:02x}, len: {})", reg, len);

        Ok(())
    }
}

/// A no-op pin, which does nothing.
///
/// This may be useful in the context of [`UARTSPI`], to provide e.g. a CS or reset pin, which does
/// nothing in this case.
pub struct NoOpPin;

impl OutputPin for NoOpPin {
    type Error = ();

    fn set_low(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}
