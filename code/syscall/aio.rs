use crate::types::*;
use crate::error::{Error, Result, result};

#[no_mangle]
pub fn iopl() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_IOPL) })
}

#[no_mangle]
pub fn ioperm() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_IOPERM) })
}
#[no_mangle]
pub fn io_setup() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_IO_SETUP) })
}

#[no_mangle]
pub fn io_destroy() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_IO_DESTROY) })
}

#[no_mangle]
pub fn io_getevents() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_IO_GETEVENTS) })
}

#[no_mangle]
pub fn io_submit() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_IO_SUBMIT) })
}

#[no_mangle]
pub fn io_cancel() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_IO_CANCEL) })
}
#[no_mangle]
pub fn ioprio_set() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_IOPRIO_SET) })
}

#[no_mangle]
pub fn ioprio_get() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_IOPRIO_GET) })
}
#[no_mangle]
pub fn io_pgetevents() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_IO_PGETEVENTS) })
}
#[no_mangle]
pub fn io_uring_setup() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_IO_URING_SETUP) })
}

#[no_mangle]
pub fn io_uring_enter() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_IO_URING_ENTER) })
}

#[no_mangle]
pub fn io_uring_register() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_IO_URING_REGISTER) })
}
