use embassy_stm32::Peripherals;
use embassy_stm32::gpio::Pin;

pub struct BoardPins {
    //Analog pins(CN8)
    pub a0: embassy_stm32::peripherals::PA0,
    pub a1: embassy_stm32::peripherals::PA1,
    pub a2: embassy_stm32::peripherals::PA4,
    pub a3: embassy_stm32::peripherals::PB0,
    pub a4: embassy_stm32::peripherals::PC1,
    pub a5: embassy_stm32::peripherals::PC0,

    //Digital pins(CN5 & CN9)
    pub d0: embassy_stm32::peripherals::PA3,
    pub d1: embassy_stm32::peripherals::PA2,
    pub d2: embassy_stm32::peripherals::PC8,
    pub d3: embassy_stm32::peripherals::PB3,
    pub d4: embassy_stm32::peripherals::PB5,
    pub d5: embassy_stm32::peripherals::PB4,
    pub d6: embassy_stm32::peripherals::PB10,
    pub d7: embassy_stm32::peripherals::PA8,
    pub d8: embassy_stm32::peripherals::PC7,
    pub d9: embassy_stm32::peripherals::PC6,
    pub d10: embassy_stm32::peripherals::PC9,
    pub d11: embassy_stm32::peripherals::PA7,
    pub d12: embassy_stm32::peripherals::PA6,
    pub d13: embassy_stm32::peripherals::PA5,
    pub d14: embassy_stm32::peripherals::PB7,
    pub d15: embassy_stm32::peripherals::PB6,

    //LEDs
    // pub user_led: embassy_stm32::peripherals::PA5,

    //Buttons
    pub user_button: embassy_stm32::peripherals::PC13,
}

impl BoardPins {
    pub fn new(peripherals: Peripherals) -> Self {
        Self {
            a0: peripherals.PA0,
            a1: peripherals.PA1,
            a2: peripherals.PA4,
            a3: peripherals.PB0,
            a4: peripherals.PC1,
            a5: peripherals.PC0,

            d0: peripherals.PA3,
            d1: peripherals.PA2,
            d2: peripherals.PC8,
            d3: peripherals.PB3,
            d4: peripherals.PB5,
            d5: peripherals.PB4,
            d6: peripherals.PB10,
            d7: peripherals.PA8,
            d8: peripherals.PC7,
            d9: peripherals.PC6,
            d10: peripherals.PC9,
            d11: peripherals.PA7,
            d12: peripherals.PA6,
            d13: peripherals.PA5,
            d14: peripherals.PB7,
            d15: peripherals.PB6,

            // user_led: peripherals.PA5,
            user_button: peripherals.PC13,
        }
    }
}
