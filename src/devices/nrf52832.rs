use super::Peripherals;
use crate::aliases::{Button, I2cConcrete, LedPin, UartConcrete};
use embassy_nrf::gpio::{Input, Level, Output, OutputDrive, Pull};
use embassy_nrf::{interrupt, twim, uarte};

pub fn peripherals() -> Peripherals<LedPin, Button, I2cConcrete, UartConcrete> {
    let p = embassy_nrf::init(Default::default());
    let led = Output::new(p.P0_17, Level::Low, OutputDrive::Standard);
    let button = Input::new(p.P0_13, Pull::Up);

    let config = twim::Config::default();
    let irq = interrupt::take!(SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0);
    let i2c = embassy_nrf::twim::Twim::new(p.TWISPI0, irq, p.P0_26, p.P0_27, config);

    let mut config = uarte::Config::default();
    config.parity = uarte::Parity::EXCLUDED;
    config.baudrate = uarte::Baudrate::BAUD115200;
    let irq = interrupt::take!(UARTE0_UART0);
    let uart = uarte::Uarte::new(p.UARTE0, irq, p.P0_08, p.P0_06, config);

    Peripherals {
        led,
        button,
        i2c,
        uart,
    }
}
