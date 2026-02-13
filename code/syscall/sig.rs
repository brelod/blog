use crate::types::*;
use crate::error::{Error, Result, result};

#[no_mangle]
pub fn pause() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_PAUSE) })
}
#[no_mangle]
pub fn alarm() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_ALARM) })
}
#[no_mangle]
pub fn kill() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_KILL) })
}
#[no_mangle]
pub fn rt_sigpending() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_RT_SIGPENDING) })
}

#[no_mangle]
pub fn rt_sigtimedwait() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_RT_SIGTIMEDWAIT) })
}

#[no_mangle]
pub fn rt_sigqueueinfo() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_RT_SIGQUEUEINFO) })
}

#[no_mangle]
pub fn rt_sigsuspend() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_RT_SIGSUSPEND) })
}

#[no_mangle]
pub fn sigaltstack() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_SIGALTSTACK) })
}
#[no_mangle]
pub fn tkill() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_TKILL) })
}
#[no_mangle]
pub fn tgkill() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_TGKILL) })
}
#[no_mangle]
pub fn signalfd() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_SIGNALFD) })
}
#[no_mangle]
pub fn signalfd4() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_SIGNALFD4) })
}
#[no_mangle]
pub fn rt_tgsigqueueinfo() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_RT_TGSIGQUEUEINFO) })
}
#[no_mangle]
pub fn userfaultfd() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_USERFAULTFD) })
}
#[no_mangle]
pub fn pidfd_send_signal() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_PIDFD_SEND_SIGNAL) })
}

