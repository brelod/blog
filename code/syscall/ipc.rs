use crate::types::*;
use crate::error::{Error, Result, result};

#[no_mangle]
pub fn pipe() -> Result<(u32, u32)> {
    let fds = [0u32; 2];
    result(unsafe { syscall!(super::SYS_PIPE, fds.as_ptr()) }).map(|_| (fds[0], fds[1]))
}
#[no_mangle]
pub fn shmget() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_SHMGET) })
}

#[no_mangle]
pub fn shmat() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_SHMAT) })
}

#[no_mangle]
pub fn shmctl() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_SHMCTL) })
}

#[no_mangle]
pub fn shmdt() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_SHMDT) })
}

#[no_mangle]
pub fn msgget() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_MSGGET) })
}

#[no_mangle]
pub fn msgsnd() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_MSGSND) })
}

#[no_mangle]
pub fn msgrcv() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_MSGRCV) })
}

#[no_mangle]
pub fn msgctl() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_MSGCTL) })
}
#[no_mangle]
pub fn mq_open() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_MQ_OPEN) })
}

#[no_mangle]
pub fn mq_unlink() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_MQ_UNLINK) })
}

#[no_mangle]
pub fn mq_timedsend() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_MQ_TIMEDSEND) })
}

#[no_mangle]
pub fn mq_timedreceive() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_MQ_TIMEDRECEIVE) })
}

#[no_mangle]
pub fn mq_notify() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_MQ_NOTIFY) })
}

#[no_mangle]
pub fn mq_getsetattr() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_MQ_GETSETATTR) })
}
#[no_mangle]
pub fn splice() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_SPLICE) })
}

#[no_mangle]
pub fn tee() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_TEE) })
}
#[no_mangle]
pub fn vmsplice() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_VMSPLICE) })
}
#[no_mangle]
pub fn pipe2() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_PIPE2) })
}
