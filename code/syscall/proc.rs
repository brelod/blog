use crate::types::*;
use crate::error::{Error, Result, result};

#[no_mangle]
pub fn clone() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_CLONE) })
}

#[no_mangle]
pub fn fork() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_FORK) })
}

#[no_mangle]
pub fn vfork() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_VFORK) })
}

#[no_mangle]
pub fn execve() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_EXECVE) })
}

#[no_mangle]
pub fn exit(rc: u8) -> ! {
    unsafe { syscall!(super::SYS_EXIT, rc as u32); }
    unreachable!();
}

#[no_mangle]
pub fn wait4() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_WAIT4) })
}
#[no_mangle]
pub fn ptrace() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_PTRACE) })
}
#[no_mangle]
pub fn gettid() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_GETTID) })
}
#[no_mangle]
pub fn set_tid_address() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_SET_TID_ADDRESS) })
}
#[no_mangle]
pub fn exit_group() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_EXIT_GROUP) })
}

#[no_mangle]
pub fn waitid() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_WAITID) })
}
#[no_mangle]
pub fn unshare() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_UNSHARE) })
}
#[no_mangle]
pub fn kcmp() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_KCMP) })
}
#[no_mangle]
pub fn execveat() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_EXECVEAT) })
}
#[no_mangle]
pub fn rseq() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_RSEQ) })
}
#[no_mangle]
pub fn pidfd_open() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_PIDFD_OPEN) })
}

#[no_mangle]
pub fn clone3() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_CLONE3) })
}

#[no_mangle]
pub fn pidfd_getfd() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_PIDFD_GETFD) })
}
