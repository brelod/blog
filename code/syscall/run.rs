use crate::types::*;
use crate::error::{Error, Result, result};

#[no_mangle]
pub fn getpid() -> pid_t {
    // SAFETY: This syscall never fails and it always returns with pid_t
    // so it's safe to use the **as** keyword.
    unsafe { syscall!(super::SYS_GETPID) as pid_t }
}
#[no_mangle]
pub fn getcwd() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_GETCWD) })
}
#[no_mangle]
pub fn chdir() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_CHDIR) })
}

#[no_mangle]
pub fn fchdir() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_FCHDIR) })
}
#[no_mangle]
pub fn getrlimit() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_GETRLIMIT) })
}
#[no_mangle]
pub fn setrlimit() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_SETRLIMIT) })
}

#[no_mangle]
pub fn prlimit64() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_PRLIMIT64) })
}

#[no_mangle]
pub fn getrusage() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_GETRUSAGE) })
}

#[no_mangle]
pub fn getuid() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_GETUID) })
}

#[no_mangle]
pub fn getgid() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_GETGID) })
}

#[no_mangle]
pub fn setuid() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_SETUID) })
}

#[no_mangle]
pub fn setgid() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_SETGID) })
}

#[no_mangle]
pub fn geteuid() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_GETEUID) })
}

#[no_mangle]
pub fn getegid() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_GETEGID) })
}

#[no_mangle]
pub fn setpgid() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_SETPGID) })
}

#[no_mangle]
pub fn getppid() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_GETPPID) })
}

#[no_mangle]
pub fn getpgrp() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_GETPGRP) })
}

#[no_mangle]
pub fn setsid() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_SETSID) })
}

#[no_mangle]
pub fn setreuid() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_SETREUID) })
}

#[no_mangle]
pub fn setregid() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_SETREGID) })
}

#[no_mangle]
pub fn getgroups() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_GETGROUPS) })
}

#[no_mangle]
pub fn setgroups() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_SETGROUPS) })
}

#[no_mangle]
pub fn setresuid() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_SETRESUID) })
}

#[no_mangle]
pub fn getresuid() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_GETRESUID) })
}

#[no_mangle]
pub fn setresgid() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_SETRESGID) })
}

#[no_mangle]
pub fn getresgid() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_GETRESGID) })
}

#[no_mangle]
pub fn getpgid() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_GETPGID) })
}

#[no_mangle]
pub fn setfsuid() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_SETFSUID) })
}

#[no_mangle]
pub fn setfsgid() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_SETFSGID) })
}

#[no_mangle]
pub fn getsid() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_GETSID) })
}

#[no_mangle]
pub fn capget() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_CAPGET) })
}

#[no_mangle]
pub fn capset() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_CAPSET) })
}
#[no_mangle]
pub fn modify_ldt() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_MODIFY_LDT) })
}

#[no_mangle]
pub fn pivot_root() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_PIVOT_ROOT) })
}
#[no_mangle]
pub fn prctl() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_PRCTL) })
}

#[no_mangle]
pub fn arch_prctl() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_ARCH_PRCTL) })
}
#[no_mangle]
pub fn chroot() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_CHROOT) })
}
#[no_mangle]
pub fn perf_event_open() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_PERF_EVENT_OPEN) })
}
#[no_mangle]
pub fn setns() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_SETNS) })
}

#[no_mangle]
pub fn getcpu() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_GETCPU) })
}
