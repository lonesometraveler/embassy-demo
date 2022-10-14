use super::Peripherals;
use crate::aliases::{Button, I2cConcrete, LedPin, SpiConcrete, SpiCs, UartConcrete};
use embassy_nrf::gpio::{Input, Level, Output, OutputDrive, Pull};
use embassy_nrf::{interrupt, spim, twim, uarte};

pub use embassy_nrf::peripherals::UARTE0 as UART;
pub use embassy_nrf::uarte::{UarteRx as UartRx, UarteTx as UartTx};

pub fn peripherals() -> Peripherals<LedPin, Button, I2cConcrete, UartConcrete, SpiConcrete, SpiCs> {
    let p = embassy_nrf::init(Default::default());
    let led = Output::new(p.P0_13, Level::Low, OutputDrive::Standard);
    let button = Input::new(p.P0_11, Pull::Up);

    let config = twim::Config::default();
    let irq = interrupt::take!(SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0);
    let i2c = embassy_nrf::twim::Twim::new(p.TWISPI0, irq, p.P0_26, p.P0_27, config);

    let mut config = uarte::Config::default();
    config.parity = uarte::Parity::EXCLUDED;
    config.baudrate = uarte::Baudrate::BAUD115200;
    let irq = interrupt::take!(UARTE0_UART0);
    let uart = uarte::Uarte::new(p.UARTE0, irq, p.P0_08, p.P0_06, config);

    let config = spim::Config::default();
    let irq = interrupt::take!(SPIM3);
    let spim = spim::Spim::new(p.SPI3, irq, p.P0_29, p.P0_28, p.P0_30, config);
    let ncs = Output::new(p.P0_31, Level::High, OutputDrive::Standard);

    Peripherals {
        led: Some(led),
        button: Some(button),
        i2c: Some(i2c),
        uart: Some(uart),
        spim: Some(spim),
        ncs: Some(ncs),
    }
}
