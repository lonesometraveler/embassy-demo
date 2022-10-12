use super::Peripherals;
use crate::aliases::{Button, LedPin};
use embassy_nrf::gpio::{Input, Level, Output, OutputDrive, Pull};

pub fn peripherals() -> Peripherals<LedPin, Button> {
    let p = embassy_nrf::init(Default::default());
    let led = Output::new(p.P0_17, Level::Low, OutputDrive::Standard);
    let button = Input::new(p.P0_13, Pull::Up);
    Peripherals { led, button }
}
