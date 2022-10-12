#[cfg(feature = "nrf52832dk")]
mod aliase {
    use embassy_nrf::gpio::{Input, Output};
    use embassy_nrf::peripherals::{P0_13, P0_17};
    pub type LedPin = Output<'static, P0_17>;
    pub type Button = Input<'static, P0_13>;
}

#[cfg(feature = "nrf52840dk")]
mod aliase {
    use embassy_nrf::gpio::{Input, Output};
    use embassy_nrf::peripherals::{P0_11, P0_13};
    pub type LedPin = Output<'static, P0_13>;
    pub type Button = Input<'static, P0_11>;
}

pub use aliase::{Button, LedPin};
