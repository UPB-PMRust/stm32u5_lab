# NUCLEO-U545RE-Q

![NUCLEO STM32U535RE-Q](./nucleo_stm32u535re-q.avif "NUCLEO STM32U535RE-Q board")

### ARDUINO® power connector (CN6) pinout

| Pin | Pin label | Signal name | STM32 pin | Additional function |
| --- | --------- | ----------- | --------- | ------------------- |
| 1   | NC        | NC          | -         | RESERVED            |
| 2   | IOREF     | IOREF       | -         | I/O REF             |
| 3   | NRST      | NRST        | NRST      | RESET               |
| 4   | 3V3       | 3V3         | -         | 3V3 input/output    |
| 5   | 5V        | 5V          | -         | 5V output           |
| 6   | GND       | GND         | -         | GND                 |
| 7   | GND       | GND         | -         | GND                 |
| 8   | VIN       | VIN         | -         | VIN (7-12 V)        |

### ARDUINO® ADC connector (CN8) pinout

| Pin | Pin label | Signal name | STM32 pin | Additional function |
| --- | --------- | ----------- | --------- | ------------------- |
| 1   | A0        | ADC         | PA0       | ADC1_IN5            |
| 2   | A1        | ADC         | PA1       | ADC1_IN6            |
| 3   | A2        | ADC         | PA4       | ADC1_IN9            |
| 4   | A3        | ADC         | PB0       | ADC1_IN15           |
| 5   | A4        | ADC/I²C     | PC1       | ADC1_IN2/I2C3_SDA   |
| 6   | A5        | ADC/I²C     | PC0       | ADC1_IN1I2C3_SCL    |

### ARDUINO® D[7-0] connector (CN9) pinout

| Pin | Pin label | Signal name | STM32 pin | Additional function |
| --- | --------- | ----------- | --------- | ------------------- |
| 1   | D7        | IO          | PA8       | I/O                 |
| 2   | D6        | PWM         | PB10      | TIM2_CH3            |
| 3   | D5        | PWM         | PB4       | TIM3_CH1            |
| 4   | D4        | IO          | PB5       | I/O                 |
| 5   | D3        | PWM         | PB3       | TIM2_CH2            |
| 6   | D2        | IO          | PC8       | I/O                 |
| 7   | D1        | USART_A_TX  | PA2       | LPUART1             |
| 8   | D0        | USART_A_RX  | PA3       | LPUART1             |

### ARDUINO® D[15-8] connector (CN5) pinout

| Pin | Pin label | Signal name  | STM32 pin | Additional function |
| --- | --------- | ------------ | --------- | ------------------- |
| 1   | D15       | I2C_SCL      | PB6       | I2C1_SCL/I2C4_SCL   |
| 2   | D14       | I2C_SDA      | PB7       | I2C1_SDA/I2C4_SDA   |
| 3   | DVREFP    | -            | -         | -                   |
| 4   | GND       | -            | -         | -                   |
| 5   | D13       | SPI_SCK      | PA5       | SPI1_SCK            |
| 6   | D12       | SPI_MISO     | PA6       | SPI1_MISO           |
| 7   | D11       | SPI_MOSI/PWM | PA7       | SPI1_MOSI/TIM3_CH2  |
| 8   | D10       | SPI_NSS/PWM  | PC9       | SPI_NSS/TIM3_CH4    |
| 9   | D9        | PWM          | PC6       | TIM3_CH1            |
| 10  | D8        | IO           | PC7       | -                   |

More informations can be found [here](https://www.st.com/en/evaluation-tools/nucleo-u545re-q.html?ecmp=tt9470_gl_link_feb2019&rt=um&id=UM3062#overview).
