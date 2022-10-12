#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

mod aliases;
mod devices;

use crate::aliases::{Button, LedPin};
use defmt::{info, unwrap};
use devices::peripherals;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = self::peripherals();
    unwrap!(spawner.spawn(ticker()));
    unwrap!(spawner.spawn(led_task(p.led)));
    unwrap!(spawner.spawn(button_task(p.button)));
    info!("app started");
}

#[embassy_executor::task]
async fn button_task(mut button: Button) {
    loop {
        button.wait_for_falling_edge().await;
        info!("Button pressed!");
        button.wait_for_rising_edge().await;
        info!("Button released!");
    }
}

#[embassy_executor::task]
async fn led_task(mut led: LedPin) {
    loop {
        Timer::after(Duration::from_millis(100)).await;
        led.set_high();
        Timer::after(Duration::from_millis(1000)).await;
        led.set_low();
    }
}

#[embassy_executor::task]
async fn ticker() {
    loop {
        info!("tick, tick...");
        Timer::after(Duration::from_ticks(64_000)).await;
    }
}
