# SM2M Transcoder Driver
This is the software driver for both decoder and encoder USB device.

# Examples

Check communication with device by sending test command:

```rust
use std::time;
use sm2m_transcoder_driver::driver::Driver;

let mut driver = Driver::new().unwrap();
let timeout = time::Duration::from_secs(1);
let device = driver.find_decoder(timeout).unwrap();

if let Some(mut device) = device {
    device.check(timeout).unwrap();
}
```
