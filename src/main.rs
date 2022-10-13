#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

mod aliases;
mod devices;

use crate::aliases::{Button, I2cConcrete, LedPin};
use defmt::{error, info, unwrap};
use devices::peripherals;
use embassy_executor::Spawner;
use embassy_nrf::uarte::{UarteRx, UarteTx};
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::channel::Channel;
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

static CHANNEL: Channel<ThreadModeRawMutex, [u8; 8], 1> = Channel::new();

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = self::peripherals();
    unwrap!(spawner.spawn(ticker()));
    unwrap!(spawner.spawn(led_task(p.led)));
    unwrap!(spawner.spawn(button_task(p.button)));
    unwrap!(spawner.spawn(i2c_sensor_task(p.i2c)));
    let (tx, rx) = p.uart.split();
    unwrap!(spawner.spawn(writer(tx)));
    unwrap!(spawner.spawn(reader(rx)));
    unwrap!(spawner.spawn(channel_task()));
    info!("app started");
}

#[embassy_executor::task]
async fn channel_task() {
    let mut buf = [0; 8];
    buf.copy_from_slice(b"Hello!\r\n");
    loop {
        CHANNEL.send(buf).await;
        Timer::after(Duration::from_secs(10)).await;
    }
}

#[embassy_executor::task]
async fn writer(mut tx: UarteTx<'static, embassy_nrf::peripherals::UARTE0>) {
    loop {
        let buf = CHANNEL.recv().await;
        unwrap!(tx.write(&buf).await);
        info!("wrote [{:?}]", buf);
    }
}

#[embassy_executor::task]
async fn reader(mut rx: UarteRx<'static, embassy_nrf::peripherals::UARTE0>) {
    let mut buf = [0; 8];
    loop {
        unwrap!(rx.read(&mut buf).await);
        info!("read [{:?}]", buf);
    }
}

#[embassy_executor::task]
async fn i2c_sensor_task(mut i2c: I2cConcrete) {
    const ADDRESS: u8 = 0x6B;
    loop {
        let mut buf = [0u8; 1];
        match i2c.write_read(ADDRESS, &[0x0F], &mut buf).await {
            Ok(_) => info!("Read: {=[u8]:x}", buf),
            Err(e) => error!("{:?}", e),
        }
        Timer::after(Duration::from_millis(5_000)).await;
    }
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
        Timer::after(Duration::from_millis(1_000)).await;
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
