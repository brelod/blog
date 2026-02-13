use crate::cstr;
use crate::types::*;
use crate::error::{Error, Result, result};

#[no_mangle]
#[doc = include_str!(".file/open.md")]
pub fn open(path: &str, flags: i32, mode: mode_t) -> Result<u32> {
    let mut dst = [0u8; crate::limits::PATH_MAX];
    cstr(path.as_bytes(), &mut dst)?;
    result(unsafe { syscall!(super::SYS_OPEN, dst.as_ptr(), flags, mode) }).map(|n| n as u32)
}

#[no_mangle]
pub fn close(fd: u32) -> Result<()> {
    result(unsafe { syscall!(super::SYS_CLOSE, fd) }).map(|_| ())
}

#[no_mangle]
pub fn stat(path: &str, stat: &mut stat64) -> Result<()> {
    let mut dst = [0u8; crate::limits::PATH_MAX];
    cstr(path.as_bytes(), &mut dst)?;
    result(unsafe { syscall!(super::SYS_NEWSTAT, dst.as_ptr(), stat) }).map(|_| ())
}

#[no_mangle]
pub fn fstat(fd: u32, stat: &mut stat64) -> Result<()> {
    result(unsafe { syscall!(super::SYS_NEWFSTAT, fd, stat) }).map(|_| ())
}

#[no_mangle]
pub fn lstat(path: &str, stat: &mut stat64) -> Result<()> {
    let mut dst = [0u8; crate::limits::PATH_MAX];
    cstr(path.as_bytes(), &mut dst)?;
    result(unsafe { syscall!(super::SYS_NEWLSTAT, dst.as_ptr(), stat) }).map(|_| ())
}
#[no_mangle]
pub fn lseek(fd: u32, offset: off_t, whence: i32) -> Result<off_t> {
    result(unsafe { syscall!(super::SYS_LSEEK, fd, offset, whence) })
}
#[no_mangle]
pub fn dup() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_DUP) })
}

#[no_mangle]
pub fn dup2() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_DUP2) })
}
#[no_mangle]
pub fn fcntl() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_FCNTL) })
}

#[no_mangle]
pub fn flock() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_FLOCK) })
}

#[no_mangle]
pub fn fsync(fd: u32) -> Result<()> {
    result(unsafe { syscall!(super::SYS_FSYNC, fd) }).map(|_| ())
}

#[no_mangle]
pub fn fdatasync(fd: u32) -> Result<()> {
    result(unsafe { syscall!(super::SYS_FDATASYNC, fd) }).map(|_| ())
}

#[no_mangle]
pub fn truncate(path: &str, len: off_t) -> Result<()> {
    let mut dst = [0u8; crate::limits::PATH_MAX];
    cstr(path.as_bytes(), &mut dst)?;
    result(unsafe { syscall!(super::SYS_TRUNCATE, dst.as_ptr(), len) }).map(|_| ())
}

#[no_mangle]
pub fn ftruncate(fd: u32, len: off_t) -> Result<()> {
    result(unsafe { syscall!(super::SYS_FTRUNCATE, fd, len) }).map(|_| ())
}
#[no_mangle]
pub fn readahead() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_READAHEAD) })
}
#[no_mangle]
pub fn fadvise64() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_FADVISE64) })
}

#[no_mangle]
pub fn openat() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_OPENAT) })
}
#[no_mangle]
pub fn fallocate(fd: u32, mode: u32, offset: off_t, len: off_t) -> Result<()> {
    result(unsafe { syscall!(super::SYS_FALLOCATE, fd, mode, offset, len) }).map(|_| ())
}
#[no_mangle]
pub fn dup3() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_DUP3) })
}

#[no_mangle]
pub fn name_to_handle_at() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_NAME_TO_HANDLE_AT) })
}

#[no_mangle]
pub fn open_by_handle_at() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_OPEN_BY_HANDLE_AT) })
}
#[no_mangle]
pub fn close_range() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_CLOSE_RANGE) })
}

#[no_mangle]
pub fn openat2() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_OPENAT2) })
}
