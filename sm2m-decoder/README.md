# SM2M Decoder
SM2M Decoder is the decoder of the SM2M computing units signals.
The firmware is developed for STM32F103 microcontroller.
It is build using the RTIC - a concurrency framework for building real-time systems.
You can find more information in the official RTIC book https://rtic.rs/0.5/book/en/.

# High level design
![High level design](design.svg)

# Build firmware binary
Here we build the firmware binary and then create `.bin` file which we can upload directly to the MCU.
```bash
cargo build --release
cargo objcopy --release -- -O binary ./target/decoder.bin
```

# Upload firmware to MCU using DFU
To upload compiled firmware you need to make sure that BOOT0 jumper connects BOOT0 to the 3v3 and BOO1 jumper connects BOOT1 to GND.
Then connect the board to USB and run the following command:
```bash
dfu-util -d 0483:df11 -a 0 -s 0x8000000 -D ./target/decoder.bin
```

Alternatively you can create the following shell file:
```bash
cargo build --release && \
cargo objcopy --release -- -O binary ./target/decoder.bin && \
dfu-util -d 0483:df11 -a 0 -s 0x8000000 -D ./target/decoder.bin
```

_Warning!
This method may not work on Chinesse made boards.
Many bluepill boards are know to have a USB pull up resistor with a value far off. It requires to replace the USB DP pull up with the right value. If it's not the case try uploading the firmware using ST-Link V2._

# Upload firmware to MCU using ST-Link V2

## ST-Link V2 USB debugger to Blue Pill board connection
| Blue Pill | ST-Link V2 |
| --- | --- |
| **V3** red | **3.3V** pin 8 |
| **IO** orange | **SWDIO** pin 4 |
| **CLK** brown | **SWDCLK** pin 2 |
| **GND** black | **GND** pin 6 |
