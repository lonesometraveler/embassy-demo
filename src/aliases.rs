#[cfg(feature = "nrf52832dk")]
mod aliase {
    use embassy_nrf::gpio::{Input, Output};
    use embassy_nrf::peripherals::{P0_13, P0_17, TWISPI0, UARTE0};
    pub type LedPin = Output<'static, P0_17>;
    pub type Button = Input<'static, P0_13>;
    pub type I2cConcrete = embassy_nrf::twim::Twim<'static, TWISPI0>;
    pub type UartConcrete = embassy_nrf::uarte::Uarte<'static, UARTE0>;
}

#[cfg(feature = "nrf52840dk")]
mod aliase {
    use embassy_nrf::gpio::{Input, Output};
    use embassy_nrf::peripherals::{P0_11, P0_13, TWISPI0, UARTE0};
    pub type LedPin = Output<'static, P0_13>;
    pub type Button = Input<'static, P0_11>;
    pub type I2cConcrete = embassy_nrf::twim::Twim<'static, TWISPI0>;
    pub type UartConcrete = embassy_nrf::uarte::Uarte<'static, UARTE0>;
}

pub use aliase::{Button, I2cConcrete, LedPin, UartConcrete};

pub trait I2c: embedded_hal::i2c::I2c {
    type Error: core::fmt::Debug;
}
impl<T: embedded_hal::i2c::I2c> I2c for T {
    type Error = T::Error;
}

pub trait Uart: embedded_hal::serial::Write {}
impl<T: embedded_hal::serial::Write> Uart for T {}
