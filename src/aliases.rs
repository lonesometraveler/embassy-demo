#[cfg(feature = "nrf52832dk")]
mod alias {
    use embassy_nrf::gpio::{Input, Output};
    use embassy_nrf::peripherals::{P0_13, P0_17, P0_20, SPI2, TWISPI0, UARTE0};
    pub type LedPin = Output<'static, P0_17>;
    pub type Button = Input<'static, P0_13>;
    pub type I2cConcrete = embassy_nrf::twim::Twim<'static, TWISPI0>;
    pub type UartConcrete = embassy_nrf::uarte::Uarte<'static, UARTE0>;
    pub type SpiConcrete = embassy_nrf::spim::Spim<'static, SPI2>;
    pub type SpiCs = Output<'static, P0_20>;
}

#[cfg(feature = "nrf52840dk")]
mod alias {
    use embassy_nrf::gpio::{Input, Output};
    use embassy_nrf::peripherals::{P0_11, P0_13, P0_31, SPI3, TWISPI0, UARTE0};
    pub type LedPin = Output<'static, P0_13>;
    pub type Button = Input<'static, P0_11>;
    pub type I2cConcrete = embassy_nrf::twim::Twim<'static, TWISPI0>;
    pub type UartConcrete = embassy_nrf::uarte::Uarte<'static, UARTE0>;
    pub type SpiConcrete = embassy_nrf::spim::Spim<'static, SPI3>;
    pub type SpiCs = Output<'static, P0_31>;
}

#[cfg(feature = "nucleo429")]
mod alias {
    use embassy_stm32::dma::NoDma;
    use embassy_stm32::exti::ExtiInput;
    use embassy_stm32::gpio::Output;
    use embassy_stm32::peripherals::{I2C1, PB7, PC13, PE0, SPI3, USART3};
    pub type LedPin = Output<'static, PB7>;
    pub type Button = ExtiInput<'static, PC13>;
    pub type I2cConcrete = embassy_stm32::i2c::I2c<'static, I2C1>;
    pub type UartConcrete = embassy_stm32::usart::Uart<'static, USART3>;
    pub type SpiConcrete = embassy_stm32::spi::Spi<'static, SPI3, NoDma, NoDma>;
    pub type SpiCs = Output<'static, PE0>;
}

pub use alias::{Button, I2cConcrete, LedPin, SpiConcrete, SpiCs, UartConcrete};

pub trait I2c: embedded_hal::i2c::I2c {
    type Error: core::fmt::Debug;
}
impl<T: embedded_hal::i2c::I2c> I2c for T {
    type Error = T::Error;
}

pub trait Uart: embedded_hal::serial::Write {}
impl<T: embedded_hal::serial::Write> Uart for T {}

pub trait Spi: embedded_hal::spi::SpiBusRead + embedded_hal::spi::SpiBusWrite {}
impl<T: embedded_hal::spi::SpiBusRead + embedded_hal::spi::SpiBusWrite> Spi for T {}
