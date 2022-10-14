use embedded_hal::digital::{InputPin, OutputPin};
#[cfg_attr(feature = "nrf52832dk", path = "nrf52832.rs")]
#[cfg_attr(feature = "nrf52840dk", path = "nrf52840.rs")]
#[cfg_attr(feature = "nucleo429", path = "nucleo429.rs")]
mod device;

use crate::aliases::{I2c, Spi, Uart};
pub use device::peripherals;
pub use device::{UartRx, UartTx, UART};

pub struct Peripherals<LED: OutputPin, BUTTON: InputPin, I: I2c, U: Uart, S: Spi, NCS: OutputPin> {
    pub led: Option<LED>,
    pub button: Option<BUTTON>,
    pub i2c: Option<I>,
    pub uart: Option<U>,
    pub spim: Option<S>,
    pub ncs: Option<NCS>,
}
