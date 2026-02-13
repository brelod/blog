use core::convert::TryInto;

#[no_mangle]
unsafe fn memset(buffer: *mut u8, byte: u8, len: usize) -> *mut u8 {
    for idx in 0..len {
        let offset = idx.try_into().unwrap();
        unsafe {
            buffer.offset(offset).write(byte);
        }
    }
    buffer
}

#[no_mangle]
unsafe fn memcpy(dst: *mut u8, src: *const u8, len: usize) -> *mut u8 {
    for idx in 0..len {
        let offset = idx.try_into().unwrap();
        unsafe {
            let byte = src.offset(offset).read();
            dst.offset(offset).write(byte);
        }
    }
    dst
}

#[no_mangle]
unsafe fn strlen(buf: *const u8) -> usize {
    let mut len = 0;
    while unsafe { *buf.offset(len) != 0 } {
        len += 1;
    }
    let x = len.try_into().unwrap();
    x
}

#[no_mangle]
unsafe fn memcmp(s1: *const u8, s2: *const u8, len: usize) -> i32 {
    for idx in 0..len {
        let offset = idx.try_into().unwrap();
        unsafe {
            let b1 = s1.offset(offset).read();
            let b2 = s2.offset(offset).read();
            if b1 != b2 {
                return (b1 as i8 - b2 as i8).into();
            }
        }
    }
    0
}

#[no_mangle]
unsafe fn bcmp(s1: *const u8, s2: *const u8, len: usize) -> i32 {
    // TODO: optimize this
    memcmp(s1, s2, len)
}
