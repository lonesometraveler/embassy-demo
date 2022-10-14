use super::Peripherals;
use crate::aliases::{Button, I2cConcrete, LedPin, SpiConcrete, SpiCs, UartConcrete};
use embassy_stm32::dma::NoDma;
use embassy_stm32::gpio::{Input, Level, Output, Pull, Speed};
use embassy_stm32::spi::Spi;
use embassy_stm32::time::Hertz;
use embassy_stm32::usart::Uart;
use embassy_stm32::{i2c, spi, usart};

pub use embassy_stm32::peripherals::USART3 as UART;
pub use embassy_stm32::usart::{UartRx, UartTx};

pub fn peripherals() -> Peripherals<LedPin, Button, I2cConcrete, UartConcrete, SpiConcrete, SpiCs> {
    let p = embassy_stm32::init(Default::default());
    let led = Output::new(p.PB7, Level::Low, Speed::High);
    let button = Input::new(p.PC13, Pull::Down);
    let button = embassy_stm32::exti::ExtiInput::new(button, p.EXTI13);

    // TODO: I2C
    // let config = i2c::Config::default();
    // let i2c = embassy_stm32::i2c::I2c::new(p.I2C1, p.PB8, p.PB9, Hertz(1_000), config);

    // TODO: UART
    // let config = usart::Config::default();
    // let uart = Uart::new(p.USART3, p.PD9, p.PD8, NoDma, NoDma, config);

    // TODO: SPI
    // let spim = Spi::new(
    //     p.SPI3,
    //     p.PC10,
    //     p.PC12,
    //     p.PC11,
    //     NoDma,
    //     NoDma,
    //     Hertz(1_000_000),
    //     spi::Config::default(),
    // );
    // let ncs = Output::new(p.PE0, Level::High, Speed::VeryHigh);

    Peripherals {
        led: Some(led),
        button: Some(button),
        i2c: None,
        uart: None,
        spim: None,
        ncs: None,
    }
}
