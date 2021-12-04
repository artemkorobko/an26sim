# SM2M Emulator
This is the MCU firmware which emulates SM2M computing unit by sending identical signals from SM2M data bus and receive signals sent from SM2M encoder. It uses RTIC framework. It's a concurrency framework for building real-time systems. You can find more information in the official RTIC book https://rtic.rs/0.5/book/en/. The target MCU is [STM32F103C8T6](https://www.st.com/en/microcontrollers-microprocessors/stm32f103c8.html).

# High level design
![High level design](../doc/sm2m-emulator.svg)

# Prerequisities
## Rust
- Install Rust toolchain by following the instructions on https://rustup.rs.
- Install the `rust-std` component `thumbv7em-none-eabihf` to cross-compile for ARM Cortex-M4 MCU using the following command:
```bash
rustup target add thumbv7em-none-eabihf
```
- Install `cargo-binutils` subcommands to invoke the LLVM tools shipped with the Rust toolchain.
```bash
cargo install cargo-binutils 
```
- Install `llvm-tools-preview` component for binary inspection.
```bash
rustup component add llvm-tools-preview
```

## ARM gcc extension for Mac
Before installing extension make sure you have updated [Homebrew](https://brew.sh) packages.
- Install ARM gcc extension and open on-chip debugger.
```bash
brew install armmbed/formulae/arm-none-eabi-gcc openocd
```
- Ensure extension has been installed
```
arm-none-eabi-gcc -v
```

## VS Build Tools for Windows
Download the Visual Studio 2019 Build tools from the Microsoft website: https://visualstudio.microsoft.com/thank-you-downloading-visual-studio/?sku=BuildTools&rel=16

During installation in the `Workloads` tab select `Desktop development with C++`. Select the following items on the `Installation details` page:
- MSVC v142 - VS 2019 C++ ...
- Windows 10 SDK ...
- C++ CMake tools for Windows

You can find more information about the embedded toolchains here https://docs.rust-embedded.org/book/intro/index.html.

# Build firmware binary
To build the release version of the firmware use the following command:
```bash
cargo build --release
```

# Upload firmware to MCU using ST-Link
Before uploading firmware to MCU ensure the size of the firmware can fit in MCU RAM.
```bash
cargo size --release
```

The output will look like this:
```
   text    data     bss     dec     hex filename
  11832       0     896   12728    31b8 sm2m-emulator
```

**Dec** column represents the total size of the firmware in bytes. This value should be less than 65535 bytes or 64 Kb. In the example above the firmware size is 12728 bytes or 12.5 Kb.

To upload compiled ELF binary with ST-Link we use `openocd` utility. The ELF itself contains flash start address so we can simply invoke the following command:
```bash
openocd -f ./openocd.cfg -c "init" -c "reset init" -c "flash write_image erase ./target/thumbv7m-none-eabi/release/sm2m-decoder" -c "shutdown"
```
Or invoke `upload.sh` script located in the project root directory.

_In case openocd fails to upload the firmware first time try pressing a `Reset` button on the board before openocd start and release it after you see console message `Info : Listening on port 3333 for gdb connections`. Next time you run openocd it should program the MCU without errors._

_After programming complete press `RESET` button on the board to start executing uploaded firmware._

## ST-Link V2 USB debugger to Blue Pill board connection
The other of pins represents the same order of pins on the Blue Pill board facing MCU and pins down.

| Blue Pill | ST-Link V2 |
| --- | --- |
| **V3** red | **3.3V** pin 8 |
| **IO** brown | **SWDIO** pin 4 |
| **CLK** white | **SWDCLK** pin 2 |
| **GND** black | **GND** pin 6 |

# STM32F103C8T6 Blue Pill pin layout
![STM32F103C8T6 Blue Pill pin layout](../doc/STM32F103C8T6.gif)

# Communication protocol
Each packet consists of 8 bits opcode and optional payload. The maximum size of the packet is 64 bytes. Packet received by MCU from host machine is called inbound. Packet sent from host machine to MCU is called outbound. Some of the inbound packets obligates host machine to receive response outbound packets.

## Inbound: Firmware version
Request firmware version. Packet length is 8 bits (1 byte) with opcode `1`. Below is the representation of the packet in little-endian byte order:

|Opcode 8 bits|
| --- |
|0000 0011|

## Outbound: Firmware version
Response firmware version. Packet length is 32 bits (4 bytes) with opcode `1`, 8 bits of major version with values between `1` and `254`, 8 bits of minor version with values between `0` and `254` and 8 bits of patch version with values between `0` and `254`. Below is the representation of the packet in little-endian byte order which contains firmware version `1.5.8`:

|Patch 8 bits|Minor 8 bits|Major 8 bits|Opcode 8 bits|
| --- | --- | --- | --- |
|0000 1000|0000 0101|0000 0001|0000 0001|

## Inbound: Set parameter
Set/override current parameter value. Packet length is 32 bits (4 bytes) with 8 bits of opcode `2`, 8 bits of parameter index starting from `0` up to `254` and 16 bits of parameter value. Below is the representation of the request in little-endian byte order which sets parameter at index `0` with value `21845`:

|Parameter value 16 bit|Index 8 bits|Opcode 8 bits|
| --- | --- | --- |
|0101 0101 0101 0101|0000 0000|0000 0010|

## Inbound: Enable generator
Enable parameter generator and set its properties. Packet length is 40 bits (5 bytes) with 8 bits of opcode `3`, 8 bits of parameter index starting from `0` up to `254`, 8 bits of generation period based on generation sequence. For example 0 - do not generate new value. 1 - generate new value each generation sequence. 2 - generate new value every second generation sequence etc. And 16 bits of parameter generator step from `0` to `65535`. Below is the representation of the request in little-endian byte order which enables generator at index `0` with period of `100` sequences and generator step `21845`:

|Generator step 16 bit|Period 8 bits|Index 8 bits|Opcode 8 bits|
| --- | --- | --- | --- |
|0101 0101 0101 0101|0110 0100|0000 0000|0000 0011|

## Inbound: Disable generator
Disable parameter generator. Packet length is 8 bits (1 byte) with 8 bits of opcode `4`. Below is the representation of the request in little-endian byte order:

|Opcode 8 bits|
| --- |
|0000 0100|

## Inbound: Start producer
Start parameters generation task. Packet length is 32 bits (4 bytes) with 8 bits of opcode `5`, 8 bits of generation frequency (FPS) starting from `0` up to `254`. Below is the representation of the request in little-endian byte order which starts generation with frequency of `20` milliseconds:

|Frequency 8 bits|Opcode 8 bits|
| --- | --- |
|0001 0100|0000 0101|
