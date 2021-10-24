# Antonov 26 Simulator
This repository contains software packages for Antonov An-26 simulator visualization system.

![An-26 Simulator](doc/sim.jpg "An-26 Simulator")

# Prerequisities
## Rust
Rust is the common language used in all software packages.
- Install Rust toolchain by follow the instructions on https://rustup.rs
- Install the `rust-std` component `thumbv7m-none-eabi` to cross-compile for ARM Cortex-M3 using the following command:
```bash
rustup target add thumbv7m-none-eabi
```
- Install `cargo-binutils` subcommands to invoke the LLVM tools shipped with the Rust toolchain.
```bash
cargo install cargo-binutils
```
- Install `llvm-tools-preview` component for binary inspection.
```bash
rustup component add llvm-tools-preview
```

## VS Build Tools for Windows only
Download the Visual Studio 2019 Build tools from the Microsoft website: https://visualstudio.microsoft.com/thank-you-downloading-visual-studio/?sku=BuildTools&rel=16

During installation in the `Workloads` tab select `Desktop development with C++`. Select the following items on the `Installation details` page:
- MSVC v142 - VS 2019 C++ ...
- Windows 10 SDK ...
- C++ CMake tools for Windows

## ARM gcc extension for MacOS only
```bash
brew install armmbed/formulae/arm-none-eabi-gcc
# ensure extension has been installed
arm-none-eabi-gcc -v
```

- Install open on-chip debugger OpenOCD and `dfu-util` which is the host side firmware download/upload utility
```bash
brew install openocd dfu-util
```

You can find more information about the embedded toolchains here https://docs.rust-embedded.org/book/intro/index.html.

## NodeJS
NodeJS is used to develope and build SM2M Transcoder.

- Install Node version manager by following the instructions on  
   - *[`MacOS, Linux`]* https://github.com/nvm-sh/nvm  
   - *[`Windows`]* https://github.com/coreybutler/nvm-windows

- Install the latest NodeJS using NVM
```bash
nvm install lts
```
**In Windows** run terminal as administrator and manually select installed version by running `nvm use <version>`. You can find installed versions using the following command `nvm ls`.

## Visual Studio Code
Visual Studio Code is the most preferred IDE for development and being used to in developing of all packages. Follow the instructions on https://code.visualstudio.com

Install the following plugins: [Better TOML](https://marketplace.visualstudio.com/items?itemName=bungcip.better-toml),
[CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb),
[crates](https://marketplace.visualstudio.com/items?itemName=serayuzgur.crates),
[rust-analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer),
[vscode-rustfmt](https://marketplace.visualstudio.com/items?itemName=statiolake.vscode-rustfmt).

You can also install optional plugins: [Test Explorer UI](https://marketplace.visualstudio.com/items?itemName=hbenl.vscode-test-explorer),
[GitLens](https://marketplace.visualstudio.com/items?itemName=eamodio.gitlens),
[Tabnine AI Code Completion](https://marketplace.visualstudio.com/items?itemName=TabNine.tabnine-vscode),
[Code Spell Checker](https://marketplace.visualstudio.com/items?itemName=streetsidesoftware.code-spell-checker).

# Project packages
[SM2M Decoder](sm2m-decoder) - SM2M signal decoder firmware for ARM MCU.  
[SM2M Transcoder](sm2m-transcoder) - UI application for testing both transcoder and decoder USB devices.  
[SM2M Transcoder Driver](sm2m-transcoder-driver) - software driver for both transcoder and decoder which used in SM2M Transcoder and X-Plan plugin.  
[X-Plane plugin](xplane-plugin) - X-Plane 11 visualization plugin.