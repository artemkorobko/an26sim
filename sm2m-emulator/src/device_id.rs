pub fn read() -> (u16, u16, u32, u32) {
    const UID_BASE: usize = 0x1FFFF7E8;
    let device_id_0_ptr = UID_BASE as *const u16;
    let device_id_1_ptr = (UID_BASE + 0x02) as *const u16;
    let device_id_2_ptr = (UID_BASE + 0x04) as *const u32;
    let device_id_3_ptr = (UID_BASE + 0x08) as *const u32;
    let id_0 = unsafe { device_id_0_ptr.read() };
    let id_1 = unsafe { device_id_1_ptr.read() };
    let id_2 = unsafe { device_id_2_ptr.read() };
    let id_3 = unsafe { device_id_3_ptr.read() };
    (id_0, id_1, id_2, id_3)
}

pub fn read_str() -> &'static str {
    fn byte2hex(byte: u8, buf: &mut [u8]) {
        const HEX_CHARS_UPPER: &[u8; 16] = b"0123456789ABCDEF";
        let high = HEX_CHARS_UPPER[((byte & 0xf0) >> 4) as usize];
        let low = HEX_CHARS_UPPER[(byte & 0x0f) as usize];
        buf[0] = high;
        buf[1] = low;
    }

    fn u16_to_hex(value: u16, buf: &mut [u8]) {
        byte2hex(value as u8, buf);
        byte2hex((value >> 8) as u8, &mut buf[2..]);
    }

    fn u32_to_hex(value: u32, buf: &mut [u8]) {
        u16_to_hex(value as u16, buf);
        u16_to_hex((value >> 16) as u16, &mut buf[4..]);
    }

    static mut DEVICE_ID_BUF: [u8; 27] = [0; 27];
    let buf = unsafe { DEVICE_ID_BUF.as_mut() };
    let (id_0, id_1, id_2, id_3) = read();

    u16_to_hex(id_0, buf);
    buf[4] = b'-';
    u16_to_hex(id_1, &mut buf[5..]);
    buf[9] = b'-';
    u32_to_hex(id_2, &mut buf[10..]);
    buf[18] = b'-';
    u32_to_hex(id_3, &mut buf[19..]);

    unsafe { core::str::from_utf8_unchecked(buf) }
}
