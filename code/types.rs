#![allow(non_camel_case_types)]
use core::ffi::CStr;

pub type mode_t = u64;
pub type off_t = usize;
pub type nfds_t = u64;
pub type time_t = i64;
pub type pid_t = i32;
pub type subseconds_t = i64;
pub type clockid_t = i32;


#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct stat64 {
    pub st_dev: u64,
    pub st_ino: u64,
    pub st_nlink: u64,
    pub st_mode: u32,
    pub st_uid: u32,
    pub st_gid: u32,
    __pad0: i32,
    pub st_rdev: u64,
    pub st_size: i64,
    pub st_blksize: i64,
    pub st_blocks: i64,
    pub st_atime: i64,
    pub st_atime_nsec: i64,
    pub st_mtime: i64,
    pub st_mtime_nsec: i64,
    pub st_ctime: i64,
    pub st_ctime_nsec: i64,
    __reserved: [i64; 3],
}

#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct pollfd {
    pub fd: i32,
    pub events: i16,
    pub revents: i16,
}

#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct timeval {
    pub tv_sec: i64,
    pub tv_usec: i64,
}

#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct timezone {
    pub tv_minuteswest: i32,
    pub tv_dsttime: i32,
}

#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct timespec {
    pub tv_sec: i64,
    pub tv_nsec: i64,
}

/*
#[repr(C)]
#[derive(Debug)]
pub(crate) struct iovec {
    pub iov_base: *mut u8,
    pub iov_len: usize,
}
*/


#[repr(C)]
#[derive(Clone, Copy)]
pub struct auxv_t {
    pub key: u64,
    pub val: u64,
}

#[derive(Debug, Clone, Copy)]
pub enum AT {
    /// End of the vector
    AT_NULL,

    /// Entry without value
    AT_IGNORE,

    /// File descriptor of the executable
    AT_EXECFD(u64),

    /// Address of the program headers
    AT_PHDR(*const u8),

    /// Size of a program header entry
    AT_PHENT(u64),

    /// Number of the program headers
    AT_PHNUM(u64),

    /// Size of a memory page
    AT_PAGESZ(u64),

    /// Base address of the interpreter
    AT_BASE(*const u8),

    /// Various flags
    AT_FLAGS(u64),

    /// Address of the entry point
    AT_ENTRY(*const u8),

    /// Not ELF binary
    AT_NOTELF(bool),

    /// Real user id
    AT_UID(u64),

    /// Effective user id
    AT_EUID(u64),

    /// Real group id
    AT_GID(u64),

    /// Effektive group id
    AT_EGID(u64),

    /// CPU architecture
    AT_PLATFORM(&'static str),

    /// Hardware capability bitmap
    AT_HWCAP(u64),

    /// Frequency of times()
    AT_CLKTCK(u64),

    /// Secure execution mode
    AT_SECURE(bool),

    /// Platform identifier
    AT_BASE_PLATFORM(&'static str),

    /// Address of 16 random bytes
    AT_RANDOM(*const u8),

    /// Extension to AT_HWCAP
    AT_HWCAP2(u64),

    /// RSEQ supported feature size
    AT_RSEQ_FEATURE_SIZE(u64),

    /// RSEQ allocation alignment
    AT_RSEQ_ALIGN(u64),

    /// Name of the executable
    AT_EXECFN(&'static str),

    /// System informations
    AT_SYSINFO(u64),

    /// Address of vDSO
    AT_SYSINFO_EHDR(*const u8),

    /// Minimal stack size of signal delivery
    AT_MINSIGSTKSZ(u64),

    /// Newly introduced AT_* which is not supported by this lib yet
    UNKNOWN(u64),
}


impl From<auxv_t> for AT {
    fn from(aux: auxv_t) -> Self {
        use crate::constants::*;
        match aux.key {
            AT_NULL => Self::AT_NULL,
            AT_IGNORE => Self::AT_IGNORE,
            AT_EXECFD => Self::AT_EXECFD(aux.val),
            AT_PHDR => Self::AT_PHDR(aux.val as *const u8),
            AT_PHENT => Self::AT_PHENT(aux.val),
            AT_PHNUM => Self::AT_PHNUM(aux.val),
            AT_PAGESZ => Self::AT_PAGESZ(aux.val),
            AT_BASE => Self::AT_BASE(aux.val as *const u8),
            AT_FLAGS => Self::AT_FLAGS(aux.val),
            AT_ENTRY => Self::AT_ENTRY(aux.val as *const u8),
            AT_NOTELF => Self::AT_NOTELF(aux.val > 0),
            AT_UID => Self::AT_UID(aux.val),
            AT_EUID => Self::AT_EUID(aux.val),
            AT_GID => Self::AT_GID(aux.val),
            AT_EGID => Self::AT_EGID(aux.val),
            AT_PLATFORM => {
                let s = unsafe { CStr::from_ptr(aux.val as *const i8).to_str().unwrap() };
                Self::AT_PLATFORM(s)
            }
            AT_HWCAP => Self::AT_HWCAP(aux.val),
            AT_HWCAP2 => Self::AT_HWCAP2(aux.val),
            AT_CLKTCK => Self::AT_CLKTCK(aux.val),
            AT_SECURE => Self::AT_SECURE(aux.val > 0),
            AT_BASE_PLATFORM => {
                let s = unsafe { CStr::from_ptr(aux.val as *const i8).to_str().unwrap() };
                Self::AT_BASE_PLATFORM(s)
            }
            AT_RANDOM => Self::AT_RANDOM(aux.val as *const u8),
            AT_RSEQ_FEATURE_SIZE => Self::AT_RSEQ_FEATURE_SIZE(aux.val),
            AT_RSEQ_ALIGN => Self::AT_RSEQ_ALIGN(aux.val),
            AT_EXECFN => {
                let s = unsafe { CStr::from_ptr(aux.val as *const i8).to_str().unwrap() };
                Self::AT_EXECFN(s)
            }
            AT_SYSINFO => Self::AT_SYSINFO(aux.val),
            AT_SYSINFO_EHDR => Self::AT_SYSINFO_EHDR(aux.val as *const u8),
            AT_MINSIGSTKSZ => Self::AT_MINSIGSTKSZ(aux.val),
            id => Self::UNKNOWN(id),
        }
    }
}


