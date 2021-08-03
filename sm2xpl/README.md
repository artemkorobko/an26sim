# SM2XPL
This is the plugin for the X-Plane 11 which manages X-Plane aircraft properties received from the An26 simulator (SM2M based computing units). This plugin is intended to be used as the visual representation of the outer world inside the existing simulator.

### To build XPL library use the following command:

```bash
cargo build --release
```

### To install plugin use the following command:
```bash
cargo build --release && cp ./target/release/libsm2.dylib /Applications/XPlane11/Resources/plugins/sm2.xpl && /Applications/XPlane11/X-Plane.app/Contents/MacOS/X-Plane
```