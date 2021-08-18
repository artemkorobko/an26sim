# SM2XPL
This is the plugin for the X-Plane 11 which manages X-Plane aircraft properties received from the An26 simulator (SM2M based computing units). The job of this plugin is to debounce, integrate and interpolate parameters read from a USB CDC decoder.

### To build XPL library use the following command:

```bash
cargo build --release
```

### To test the plugin:
- **For MacOS** copy `XPLM.framework` and `XPWidgets.framework` from the X-Plane SDK into the project root directory where `cargo.toml` is located.

Run the following command
```bash
cargo test
```

### To test the plugin inside XPlane 11 use the following command:
For MacOs:
```bash
cargo build
mkdir -p /Applications/XPlane11/Resources/plugins/sm2m
cp ./target/debug/libsm2.dylib /Applications/XPlane11/Resources/plugins/sm2m/mac.xpl
/Applications/XPlane11/X-Plane.app/Contents/MacOS/X-Plane
```