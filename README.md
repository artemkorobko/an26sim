# Antonov 26 Simulator
This repository contains software that implements Visualization system for Antonov An-26 simulator.

![An-26 Simulator](doc/sim.jpg "An-26 Simulator")

# Prerequisities
## Rust
- Install Rust toolchain by follow the instructions on https://rustup.rs
- Install the `rust-std` component `thumbv7m-none-eabi` to cross-compile for ARM Cortex-M3 using the following command:
```bash
rustup target add thumbv7m-none-eabi
```
- Install `cargo-binutils`
```bash
cargo install cargo-binutils
```
- Install `llvm-tools-preview`
```bash
rustup component add llvm-tools-preview
```
- Install GCC extension
```bash
brew install armmbed/formulae/arm-none-eabi-gcc
# ensure extension has been installed
arm-none-eabi-gcc -v
```
- Install OpenOCD
```bash
brew install openocd
```

You can find more information about the embedded toolchains here https://docs.rust-embedded.org/book/intro/index.html.

## Visual Studio Code
To install vscode follow the instructions on https://code.visualstudio.com

Install thr following vscode plugins: [Better TOML](https://marketplace.visualstudio.com/items?itemName=bungcip.better-toml),
[CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb),
[crates](https://marketplace.visualstudio.com/items?itemName=serayuzgur.crates),
[rust-analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer),
[vscode-rustfmt](https://marketplace.visualstudio.com/items?itemName=statiolake.vscode-rustfmt).

Optional plugins: [Test Explorer UI](https://marketplace.visualstudio.com/items?itemName=hbenl.vscode-test-explorer),
[GitLens](https://marketplace.visualstudio.com/items?itemName=eamodio.gitlens),
[Tabnine AI Code Completion](https://marketplace.visualstudio.com/items?itemName=TabNine.tabnine-vscode),
[Code Spell Checker](https://marketplace.visualstudio.com/items?itemName=streetsidesoftware.code-spell-checker).

# Workspace crates
[SM2M Decoder](sm2m-decoder) - SM2M signal decoder firmware  
[X-Plane plugin](xplane-plugin) - X-Plane 11 visualization plugin