use crate::types::*;
use crate::error::{Error, Result, result};

#[no_mangle]
pub fn semget() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_SEMGET) })
}

#[no_mangle]
pub fn semop() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_SEMOP) })
}

#[no_mangle]
pub fn semctl() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_SEMCTL) })
}
#[no_mangle]
pub fn futex() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_FUTEX) })
}
#[no_mangle]
pub fn semtimedop() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_SEMTIMEDOP) })
}

#[no_mangle]
pub fn set_robust_list() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_SET_ROBUST_LIST) })
}

#[no_mangle]
pub fn get_robust_list() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_GET_ROBUST_LIST) })
}
#[no_mangle]
pub fn futex_waitv() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_FUTEX_WAITV) })
}
#[no_mangle]
pub fn futex_wake() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_FUTEX_WAKE) })
}

#[no_mangle]
pub fn futex_wait() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_FUTEX_WAIT) })
}

#[no_mangle]
pub fn futex_requeue() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_FUTEX_REQUEUE) })
}
