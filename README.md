# NUCLEO-U545RE-Q

### ARDUINO® power connector (CN6) pinout


| Pin           | Pin name      | Signal name | STM32 pin | MCU function |
| ------------- | ------------- | ----------- | --------- | ------------ |
| 1             | NC            | NC          | -         | RESERVED     |
| 2             | IOREF         | IOREF       | -         | I/O REF      |
| 3             | NRST          | NRST        | NRST      | RESET        |
| 4             | 3V3           | 3V3         | -         | 3V3 input/output|
| 5             | 5V            | 5V          | -         | 5V output       |
| 6             | GND           | GND         | -         | GND             |
| 7             | GND           | GND         | -         | GND             |
| 8             | VIN           | VIN         | -         | VIN (7-12 V)    |

### ARDUINO® ADC connector (CN8) pinout

| Pin           | Pin name      | Signal name | STM32 pin | MCU function |
| ------------- | ------------- | ----------- | --------- | ------------ |
| 1             | A0            | ADC         | PA0         | ADC1_IN5   |
| 2             | A1            | ADC         | PA1         | ADC1_IN6   |
| 3             | A2            | ADC         | PA4         | ADC1_IN9   |
| 4             | A3            | ADC         | PB0         | ADC1_IN15  |
| 5             | A4            | ADC/I²C     | PC1         | ADC1_IN2/I2C3_SDA|
| 6             | A5            | ADC/I²C     | PC0         | ADC1_IN1/I2C3_SCL|             |