use crate::types::*;
use crate::error::{Error, Result, result};

#[no_mangle]
pub fn poll(fds: &[pollfd], nfds: nfds_t, timeout: i32) -> Result<()> {
    result(unsafe { syscall!(super::SYS_POLL, nfds, timeout) }).map(|_| ())
}
#[no_mangle]
pub fn select() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_SELECT) })
}

#[no_mangle]
pub fn epoll_create() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_EPOLL_CREATE) })
}
#[no_mangle]
pub fn epoll_wait() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_EPOLL_WAIT) })
}

#[no_mangle]
pub fn epoll_ctl() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_EPOLL_CTL) })
}

#[no_mangle]
pub fn inotify_init() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_INOTIFY_INIT) })
}

#[no_mangle]
pub fn inotify_add_watch() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_INOTIFY_ADD_WATCH) })
}

#[no_mangle]
pub fn inotify_rm_watch() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_INOTIFY_RM_WATCH) })
}
#[no_mangle]
pub fn inotify_init1() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_INOTIFY_INIT1) })
}

#[no_mangle]
pub fn pselect6() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_PSELECT6) })
}

#[no_mangle]
pub fn ppoll() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_PPOLL) })
}

#[no_mangle]
pub fn epoll_pwait() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_EPOLL_PWAIT) })
}

#[no_mangle]
pub fn eventfd() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_EVENTFD) })
}
#[no_mangle]
pub fn eventfd2() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_EVENTFD2) })
}
#[no_mangle]
pub fn epoll_create1() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_EPOLL_CREATE1) })
}
#[no_mangle]
pub fn fanotify_init() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_FANOTIFY_INIT) })
}

#[no_mangle]
pub fn fanotify_mark() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_FANOTIFY_MARK) })
}

#[no_mangle]
pub fn epoll_pwait2() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_EPOLL_PWAIT2) })
}

