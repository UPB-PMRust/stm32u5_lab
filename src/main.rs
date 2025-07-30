#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
use embassy_stm32::{self as _, Config};

use defmt_rtt as _;
use embassy_time::Timer;
use panic_probe as _;

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let p = embassy_stm32::init(Default::default());
    // let pins = nucleo_stm32u545re_q::BoardPins::new(p);
    info!("Hello World!");

    let d13_pin: nucleo_stm32u545re_q::D13 = p.PA5; // Adjust this pin according to your board's LED pin

    // replace PC13 with the right pin for your board.
    let mut led = Output::new(d13_pin, Level::Low, Speed::Medium);

    loop {
        info!("Hello");
        Timer::after_secs(1).await;
    }
}
