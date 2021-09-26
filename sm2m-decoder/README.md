# SM2M Decoder
SM2M Decoder is the decoder of the SM2M computing units signals.
The firmware is developed for STM32F103 microcontroller.
It is build using the RTIC - a concurrency framework for building real-time systems.
You can find more information in the official RTIC book https://rtic.rs/0.5/book/en/.

# High level design
![High level design](design.svg)

# ST-Link V2 USB debugger to Blue Pill board connection
| Blue Pill | ST-Link V2 |
| --- | --- |
| **V3** red | **3.3V** pin 8 |
| **IO** orange | **SWDIO** pin 4 |
| **CLK** brown | **SWDCLK** pin 2 |
| **GND** black | **GND** pin 6 |

# Build firmware binary
Here we build the firmware binary and then create `.bin` file which we can upload directly to the MCU.
```bash
cargo build --release
cargo objcopy --release -- -O binary decoder.bin
```
