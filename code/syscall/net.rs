use crate::types::*;
use crate::error::{Error, Result, result};

#[no_mangle]
pub fn socket() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_SOCKET) })
}

#[no_mangle]
pub fn connect() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_CONNECT) })
}

#[no_mangle]
pub fn accept() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_ACCEPT) })
}

#[no_mangle]
pub fn sendto() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_SENDTO) })
}

#[no_mangle]
pub fn recvfrom() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_RECVFROM) })
}

#[no_mangle]
pub fn sendmsg() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_SENDMSG) })
}

#[no_mangle]
pub fn recvmsg() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_RECVMSG) })
}

#[no_mangle]
pub fn shutdown() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_SHUTDOWN) })
}

#[no_mangle]
pub fn bind() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_BIND) })
}

#[no_mangle]
pub fn listen() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_LISTEN) })
}

#[no_mangle]
pub fn getsockname() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_GETSOCKNAME) })
}

#[no_mangle]
pub fn getpeername() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_GETPEERNAME) })
}

#[no_mangle]
pub fn socketpair() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_SOCKETPAIR) })
}

#[no_mangle]
pub fn setsockopt() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_SETSOCKOPT) })
}

#[no_mangle]
pub fn getsockopt() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_GETSOCKOPT) })
}
#[no_mangle]
pub fn accept4() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_ACCEPT4) })
}
#[no_mangle]
pub fn recvmmsg() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_RECVMMSG) })
}
#[no_mangle]
pub fn sendmmsg() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_SENDMMSG) })
}
#[no_mangle]
pub fn bpf() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_BPF) })
}
