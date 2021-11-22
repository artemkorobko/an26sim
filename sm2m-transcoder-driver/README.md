# SM2M Transcoder Driver
This is the software driver for both decoder and encoder USB device.

# Examples

Check communication with device by sending test command:

```rust
use std::time;
use sm2m_transcoder_driver::{
    driver::Driver,
    protocol::{Request, Response, SM2MDevice}
},

let mut driver = Driver::new().unwrap();
let timeout = time::Duration::from_secs(1);
let device = driver.find_decoder(timeout).unwrap();

if let Some(mut device) = device {
    device.reset().unwrap();
    device.write_request(Request::Ping(1, 1)).unwrap();
    let response = device.read_response().unwrap();
    assert_eq!(Response::Pong(2, 1), response);
}
```
