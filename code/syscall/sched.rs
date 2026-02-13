use crate::types::*;
use crate::error::{Error, Result, result};

#[inline(always)]
pub fn sched_yield() {
    unsafe { syscall!(super::SYS_SCHED_YIELD) };
}

#[no_mangle]
pub fn getpriority() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_GETPRIORITY) })
}

#[no_mangle]
pub fn setpriority() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_SETPRIORITY) })
}

#[no_mangle]
pub fn sched_setparam() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_SCHED_SETPARAM) })
}

#[no_mangle]
pub fn sched_getparam() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_SCHED_GETPARAM) })
}

#[no_mangle]
pub fn sched_setscheduler() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_SCHED_SETSCHEDULER) })
}

#[no_mangle]
pub fn sched_getscheduler() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_SCHED_GETSCHEDULER) })
}

#[no_mangle]
pub fn sched_get_priority_max() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_SCHED_GET_PRIORITY_MAX) })
}

#[no_mangle]
pub fn sched_get_priority_min() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_SCHED_GET_PRIORITY_MIN) })
}

#[no_mangle]
pub fn sched_rr_get_interval() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_SCHED_RR_GET_INTERVAL) })
}
#[no_mangle]
pub fn sched_setaffinity() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_SCHED_SETAFFINITY) })
}

#[no_mangle]
pub fn sched_getaffinity() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_SCHED_GETAFFINITY) })
}
#[no_mangle]
pub fn sched_setattr() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_SCHED_SETATTR) })
}

#[no_mangle]
pub fn sched_getattr() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_SCHED_GETATTR) })
}
