#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

mod nucleo_u545re_q;

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let p = embassy_stm32::init(Default::default());
    info!("Hello World!");

    let d13_pin: nucleo_u545re_q::D4 = p.PB5; 

    let mut led = Output::new(d13_pins, Level::Low, Speed::Medium);

    loop {
        defmt::info!("on!");
        led.set_low();
        Timer::after_millis(200).await;

        defmt::info!("off!");
        led.set_high();
        Timer::after_millis(200).await;
    }
}
