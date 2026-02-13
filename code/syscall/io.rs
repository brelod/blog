use crate::types::*;
use crate::error::{Error, Result, result};

#[no_mangle]
pub fn read(fd: u32, buf: &mut [u8]) -> Result<usize> {
    result(unsafe { syscall!(super::SYS_READ, fd, buf.as_ptr(), buf.len()) })
}

#[no_mangle]
pub fn write(fd: u32, buf: &[u8]) -> Result<usize> {
    result(unsafe { syscall!(super::SYS_WRITE, fd, buf.as_ptr(), buf.len()) })
}

#[no_mangle]
pub fn pread64(fd: u32, buf: &mut [u8], off: off_t) -> Result<usize> {
    result(unsafe { syscall!(super::SYS_PREAD64, fd, buf.as_ptr(), buf.len(), off) })
}

#[no_mangle]
pub fn pwrite64(fd: u32, buf: &mut [u8], off: off_t) -> Result<usize> {
    result(unsafe { syscall!(super::SYS_PWRITE64, fd, buf.as_ptr(), buf.len(), off) })
}

// ============================================================================
#[no_mangle]
pub fn readv(fd: u32, iov: &[&mut [u8]]) -> Result<usize> {
    unimplemented!()
    // TODO: https://doc.rust-lang.org/nightly/std/io/struct.IoSliceMut.html
    //result(unsafe { syscall!(super::SYS_READV, fd, iov.as_ptr(), iov.len()) })
}

#[no_mangle]
pub fn writev(fd: u32, iov: &[&[u8]]) -> Result<usize> {
    unimplemented!()
    //result(unsafe { syscall!(super::SYS_WRITE, fd, iov.as_ptr(), iov.len()) })
}
#[no_mangle]
pub fn sendfile64() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_SENDFILE64) })
}
#[no_mangle]
pub fn sync_file_range() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_SYNC_FILE_RANGE) })
}
#[no_mangle]
pub fn preadv() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_PREADV) })
}

#[no_mangle]
pub fn pwritev() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_PWRITEV) })
}
#[no_mangle]
pub fn copy_file_range(fd_in: u32, off_in: off_t, fd_out: u32, off_out: off_t, len: usize, flags: u32) -> Result<usize> {
    result(unsafe { syscall!(super::SYS_COPY_FILE_RANGE, fd_in, 
        core::ptr::null::<off_t>(), 
        fd_out, core::ptr::null::<off_t>(), len, flags) })
}

#[no_mangle]
pub fn preadv2() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_PREADV2) })
}

#[no_mangle]
pub fn pwritev2() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_PWRITEV2) })
}
