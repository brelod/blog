use crate::types::*;
use crate::error::{Error, Result, result};

#[no_mangle]
pub fn seccomp() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_SECCOMP) })
}

#[no_mangle]
pub fn getrandom() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_GETRANDOM) })
}
#[no_mangle]
pub fn landlock_create_ruleset() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_LANDLOCK_CREATE_RULESET) })
}

#[no_mangle]
pub fn landlock_add_rule() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_LANDLOCK_ADD_RULE) })
}

#[no_mangle]
pub fn landlock_restrict_self() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_LANDLOCK_RESTRICT_SELF) })
}
#[no_mangle]
pub fn lsm_get_self_attr() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_LSM_GET_SELF_ATTR) })
}

#[no_mangle]
pub fn lsm_set_self_attr() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_LSM_SET_SELF_ATTR) })
}

#[no_mangle]
pub fn lsm_list_modulesv() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_LSM_LIST_MODULESV) })
}

