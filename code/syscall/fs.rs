use crate::cstr;
use crate::types::*;
use crate::error::{Error, Result, result};

#[no_mangle]
pub fn access(path: &str, mode: i32) -> Result<()> {
    let mut dst = [0u8; crate::limits::PATH_MAX];
    cstr(path.as_bytes(), &mut dst)?;
    result(unsafe { syscall!(super::SYS_ACCESS, dst.as_ptr(), mode) }).map(|_| ())
}
#[no_mangle]
pub fn uname() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_NEWUNAME) })
}
#[no_mangle]
pub fn getdents() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_GETDENTS) })
}
#[no_mangle]
pub fn rename() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_RENAME) })
}

#[no_mangle]
pub fn mkdir() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_MKDIR) })
}

#[no_mangle]
pub fn rmdir() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_RMDIR) })
}

#[no_mangle]
pub fn creat() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_CREAT) })
}

#[no_mangle]
pub fn link() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_LINK) })
}

#[no_mangle]
pub fn unlink() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_UNLINK) })
}

#[no_mangle]
pub fn symlink() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_SYMLINK) })
}

#[no_mangle]
pub fn readlink() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_READLINK) })
}

#[no_mangle]
pub fn chmod() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_CHMOD) })
}

#[no_mangle]
pub fn fchmod() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_FCHMOD) })
}

#[no_mangle]
pub fn chown() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_CHOWN) })
}

#[no_mangle]
pub fn fchown() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_FCHOWN) })
}

#[no_mangle]
pub fn lchown() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_LCHOWN) })
}

#[no_mangle]
pub fn umask() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_UMASK) })
}
#[no_mangle]
pub fn mknod() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_MKNOD) })
}
#[no_mangle]
pub fn ustat() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_USTAT) })
}

#[no_mangle]
pub fn statfs() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_STATFS) })
}

#[no_mangle]
pub fn fstatfs() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_FSTATFS) })
}

#[no_mangle]
pub fn sysfs() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_SYSFS) })
}
#[no_mangle]
pub fn sync() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_SYNC) })
}
#[no_mangle]
pub fn mount() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_MOUNT) })
}

#[no_mangle]
pub fn umount() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_UMOUNT) })
}
#[no_mangle]
pub fn setxattr() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_SETXATTR) })
}

#[no_mangle]
pub fn lsetxattr() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_LSETXATTR) })
}

#[no_mangle]
pub fn fsetxattr() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_FSETXATTR) })
}

#[no_mangle]
pub fn getxattr() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_GETXATTR) })
}

#[no_mangle]
pub fn lgetxattr() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_LGETXATTR) })
}

#[no_mangle]
pub fn fgetxattr() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_FGETXATTR) })
}

#[no_mangle]
pub fn listxattr() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_LISTXATTR) })
}

#[no_mangle]
pub fn llistxattr() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_LLISTXATTR) })
}

#[no_mangle]
pub fn flistxattr() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_FLISTXATTR) })
}

#[no_mangle]
pub fn removexattr() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_REMOVEXATTR) })
}

#[no_mangle]
pub fn lremovexattr() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_LREMOVEXATTR) })
}

#[no_mangle]
pub fn fremovexattr() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_FREMOVEXATTR) })
}
#[no_mangle]
pub fn getdents64() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_GETDENTS64) })
}
#[no_mangle]
pub fn mkdirat() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_MKDIRAT) })
}

#[no_mangle]
pub fn mknodat() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_MKNODAT) })
}

#[no_mangle]
pub fn fchownat() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_FCHOWNAT) })
}

#[no_mangle]
pub fn futimesat() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_FUTIMESAT) })
}

#[no_mangle]
pub fn newfstatat() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_NEWFSTATAT) })
}

#[no_mangle]
pub fn unlinkat() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_UNLINKAT) })
}

#[no_mangle]
pub fn renameat() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_RENAMEAT) })
}

#[no_mangle]
pub fn linkat() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_LINKAT) })
}

#[no_mangle]
pub fn symlinkat() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_SYMLINKAT) })
}

#[no_mangle]
pub fn readlinkat() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_READLINKAT) })
}

#[no_mangle]
pub fn fchmodat() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_FCHMODAT) })
}

#[no_mangle]
pub fn faccessat() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_FACCESSAT) })
}

#[no_mangle]
pub fn utimensat() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_UTIMENSAT) })
}
#[no_mangle]
pub fn syncfs() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_SYNCFS) })
}
#[no_mangle]
pub fn renameat2() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_RENAMEAT2) })
}
#[no_mangle]
pub fn statx() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_STATX) })
}
#[no_mangle]
pub fn open_tree() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_OPEN_TREE) })
}

#[no_mangle]
pub fn move_mount() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_MOVE_MOUNT) })
}

#[no_mangle]
pub fn fsopen() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_FSOPEN) })
}

#[no_mangle]
pub fn fsconfig() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_FSCONFIG) })
}

#[no_mangle]
pub fn fsmount() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_FSMOUNT) })
}

#[no_mangle]
pub fn fspick() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_FSPICK) })
}
#[no_mangle]
pub fn faccessat2() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_FACCESSAT2) })
}

#[no_mangle]
pub fn mount_setattr() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_MOUNT_SETATTR) })
}
#[no_mangle]
pub fn fchmodat2() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_FCHMODAT2) })
}
#[no_mangle]
pub fn statmount() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_STATMOUNT) })
}

#[no_mangle]
pub fn listmount() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_LISTMOUNT) })
}
