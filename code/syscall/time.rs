use crate::types::*;
use crate::error::{Error, Result, result};

#[no_mangle]
pub fn nanosleep(req: &timespec, rem: &mut timespec) -> Result<usize> {
    result(unsafe { syscall!(super::SYS_NANOSLEEP, req as *const _, rem as *mut _) })
}

#[no_mangle]
pub fn getitimer() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_GETITIMER) })
}
#[no_mangle]
pub fn setitimer() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_SETITIMER) })
}
#[no_mangle]
pub fn gettimeofday() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_GETTIMEOFDAY) })
}
#[no_mangle]
pub fn times() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_TIMES) })
}

#[no_mangle]
pub fn utime() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_UTIME) })
}

#[no_mangle]
pub fn adjtimex() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_ADJTIMEX) })
}
#[no_mangle]
pub fn settimeofday() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_SETTIMEOFDAY) })
}
#[no_mangle]
pub fn time() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_TIME, 0) })
}
#[no_mangle]
pub fn timer_create() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_TIMER_CREATE) })
}

#[no_mangle]
pub fn timer_settime() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_TIMER_SETTIME) })
}

#[no_mangle]
pub fn timer_gettime() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_TIMER_GETTIME) })
}

#[no_mangle]
pub fn timer_getoverrun() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_TIMER_GETOVERRUN) })
}

#[no_mangle]
pub fn timer_delete() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_TIMER_DELETE) })
}

#[no_mangle]
pub fn clock_settime() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_CLOCK_SETTIME) })
}

#[no_mangle]
pub fn clock_gettime() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_CLOCK_GETTIME) })
}

#[no_mangle]
pub fn clock_getres() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_CLOCK_GETRES) })
}

#[no_mangle]
pub fn clock_nanosleep() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_CLOCK_NANOSLEEP) })
}
#[no_mangle]
pub fn utimes() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_UTIMES) })
}
#[no_mangle]
pub fn timerfd_create() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_TIMERFD_CREATE) })
}
#[no_mangle]
pub fn timerfd_settime() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_TIMERFD_SETTIME) })
}

#[no_mangle]
pub fn timerfd_gettime() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_TIMERFD_GETTIME) })
}
#[no_mangle]
pub fn clock_adjtime() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_CLOCK_ADJTIME) })
}
