#![deny(unsafe_code)]
#![no_main]
#![no_std]

//! Example for using the SPI over UART interface.
//!
//! ## STM32F411RE - Nucleo 64
//!

#[cfg(not(feature = "env_logging"))]
use panic_rtt_target as _;
use rtt_target::rtt_init_print;

#[cfg(any(feature = "stm32f7xx", feature = "stm32f4xx"))]
use cortex_m_rt::entry;

#[cfg(feature = "stm32f4xx")]
use stm32f4 as _;
#[cfg(feature = "stm32f4xx")]
use stm32f4xx_hal as hal;
#[cfg(feature = "stm32f4xx")]
use stm32f4xx_hal::serial::{config::Config, Serial};
#[cfg(feature = "stm32f4xx")]
use stm32f4xx_hal::stm32::Peripherals as DevicePeripherals;

#[cfg(feature = "stm32f7xx")]
use stm32f7 as _;
#[cfg(feature = "stm32f7xx")]
use stm32f7xx_hal as hal;
#[cfg(feature = "stm32f7xx")]
use stm32f7xx_hal::device::Peripherals as DevicePeripherals;

use hal::serial::config::{Parity, StopBits, WordLength};
use hal::{
    delay::Delay,
    gpio::GpioExt,
    rcc::RccExt,
    time::{Bps, U32Ext},
};

use drogue_grove_uart_spi::{NoOpPin, UARTSPI};

use log::LevelFilter;
use rtt_logger::RTTLogger;
use sx127x_lora::LoRa;

const FREQUENCY: i64 = 868;
static LOGGER: RTTLogger = RTTLogger::new(LevelFilter::Info);

#[entry]
fn main() -> ! {
    rtt_init_print!(NoBlockSkip, 4096);
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(log::LevelFilter::Trace);
    log::info!("Starting up...");

    let p = DevicePeripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    #[cfg(feature = "stm32f4xx")]
    let rcc = p.RCC.constrain();
    #[cfg(feature = "stm32f7xx")]
    let mut rcc = p.RCC.constrain();

    #[cfg(feature = "stm32f4xx")]
    let clocks = rcc.cfgr.sysclk(100.mhz()).freeze();
    #[cfg(feature = "stm32f7xx")]
    let clocks = rcc.cfgr.sysclk(216.mhz()).freeze();

    #[cfg(feature = "stm32f4xx")]
    let gpioa = p.GPIOA.split();

    // delay implementation

    let delay = Delay::new(cp.SYST, clocks);

    // init

    #[cfg(feature = "stm32f4xx")]
    let serial = {
        let rx = gpioa.pa12.into_alternate_af8();
        let tx = gpioa.pa11.into_alternate_af8();
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

    log::info!("Init UARTSPI");

    let spi = UARTSPI::new(serial);

    // look, you can get it back
    // let _serial = spi.free();

    log::info!("Init LoRa");

    let lora = LoRa::new(spi, NoOpPin, NoOpPin, FREQUENCY, delay).unwrap();

    log::info!("Loop");

    loop {
        cortex_m::asm::nop();
    }
}
