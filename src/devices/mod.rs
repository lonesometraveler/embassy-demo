use embedded_hal::digital::{InputPin, OutputPin};

#[cfg(feature = "nrf52832dk")]
#[path = "nrf52832.rs"]
mod device;

#[cfg(feature = "nrf52840dk")]
#[path = "nrf52840.rs"]
mod device;

use crate::aliases::{I2c, Uart};
pub use device::peripherals;

pub struct Peripherals<LED: OutputPin, BUTTON: InputPin, I: I2c, U: Uart> {
    pub led: LED,
    pub button: BUTTON,
    pub i2c: I,
    pub uart: U,
}
