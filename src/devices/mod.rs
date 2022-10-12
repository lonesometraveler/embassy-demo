use embedded_hal::digital::v2::{InputPin, OutputPin};

#[cfg(feature = "nrf52832dk")]
#[path = "nrf52832.rs"]
mod device;

#[cfg(feature = "nrf52840dk")]
#[path = "nrf52840.rs"]
mod device;

pub use device::peripherals;

pub struct Peripherals<LED: OutputPin, BUTTON: InputPin> {
    pub led: LED,
    pub button: BUTTON,
}
