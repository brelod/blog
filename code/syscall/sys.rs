use crate::types::*;
use crate::error::{Error, Result, result};

#[no_mangle]
pub fn sysinfo() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_SYSINFO) })
}
#[no_mangle]
pub fn syslog() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_SYSLOG) })
}
#[no_mangle]
pub fn personality() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_PERSONALITY) })
}
#[no_mangle]
pub fn vhangup() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_VHANGUP) })
}
#[no_mangle]
pub fn ni_syscall() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_NI_SYSCALL) })
}

#[no_mangle]
pub fn acct() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_ACCT) })
}
#[no_mangle]
pub fn swapon() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_SWAPON) })
}

#[no_mangle]
pub fn swapoff() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_SWAPOFF) })
}

#[no_mangle]
pub fn reboot() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_REBOOT) })
}

#[no_mangle]
pub fn sethostname() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_SETHOSTNAME) })
}

#[no_mangle]
pub fn setdomainname() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_SETDOMAINNAME) })
}

#[no_mangle]
pub fn init_module() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_INIT_MODULE) })
}

#[no_mangle]
pub fn delete_module() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_DELETE_MODULE) })
}

#[no_mangle]
pub fn quotactl() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_QUOTACTL) })
}
#[no_mangle]
pub fn quotactl_fd() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_QUOTACTL_FD) })
}
#[no_mangle]
pub fn restart_syscall() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_RESTART_SYSCALL) })
}
#[no_mangle]
pub fn kexec_load() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_KEXEC_LOAD) })
}
#[no_mangle]
pub fn finit_module() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_FINIT_MODULE) })
}

#[no_mangle]
pub fn kexec_file_load() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_KEXEC_FILE_LOAD) })
}
