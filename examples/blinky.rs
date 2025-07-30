#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

mod nucleo_u545re_q; // Import the correct board module.

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let p = embassy_stm32::init(Default::default());
    info!("Hello World!");

    let d13_pin: nucleo_u545re_q::D13 = p.PA5; // Adjust this pin according to your board's LED pin

    // replace PC13 with the right pin for your board.
    let mut led = Output::new(d13_pin, Level::Low, Speed::Medium);

    loop {
        defmt::info!("on!");
        led.set_low();
        Timer::after_millis(200).await;

        defmt::info!("off!");
        led.set_high();
        Timer::after_millis(200).await;
    }
}
