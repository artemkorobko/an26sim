#!/bin/sh

cargo build --release && \
cargo objcopy --release -- -O binary ./target/decoder.bin && \
dfu-util -d 0483:df11 -a 0 -s 0x8000000 -D ./target/decoder.bin
