use embassy_stm32::peripherals;

//Analog pins(CN8)
pub type A0 = embassy_stm32::peripherals::PA0;
pub type A1 = embassy_stm32::peripherals::PA1;
pub type A2 = embassy_stm32::peripherals::PA4;
pub type A3 = embassy_stm32::peripherals::PB0;
pub type A4 = embassy_stm32::peripherals::PC1;
pub type A5 = embassy_stm32::peripherals::PC0;

//Digital pins(CN5 & CN9)
pub type D0 = embassy_stm32::peripherals::PA3;
pub type D1 = embassy_stm32::peripherals::PA2;
pub type D2 = embassy_stm32::peripherals::PC8;
pub type D3 = embassy_stm32::peripherals::PB3;
pub type D4 = embassy_stm32::peripherals::PB5;
pub type D5 = embassy_stm32::peripherals::PB4;
pub type D6 = embassy_stm32::peripherals::PB10;
pub type D7 = embassy_stm32::peripherals::PA8;
pub type D8 = embassy_stm32::peripherals::PC7;
pub type D9 = embassy_stm32::peripherals::PC6;
pub type D10 = embassy_stm32::peripherals::PC9;
pub type D11 = embassy_stm32::peripherals::PA7;
pub type D12 = embassy_stm32::peripherals::PA6;
pub type D13 = embassy_stm32::peripherals::PA5;
pub type D14 = embassy_stm32::peripherals::PB7;
pub type D15 = embassy_stm32::peripherals::PB6;

//LEDs
// pub user_led: embassy_stm32::peripherals::PA5,

//Buttons
pub type user_button = embassy_stm32::peripherals::PC13;
