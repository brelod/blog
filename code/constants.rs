use crate::types::*;

pub const O_ACCMODE:   i32 = 0o0000003;
pub const O_RDONLY:    i32 = 0o0000000;
pub const O_WRONLY:    i32 = 0o0000001;
pub const O_RDWR:      i32 = 0o0000002;
pub const O_CREAT:     i32 = 0o0000100;
pub const O_EXCL:      i32 = 0o0000200;
pub const O_NOCTTY:    i32 = 0o0000400;
pub const O_TRUNC:     i32 = 0o0001000;
pub const O_APPEND:    i32 = 0o0002000;
pub const O_NONBLOCK:  i32 = 0o0004000;
pub const O_DSYNC:     i32 = 0o0010000;
pub const O_DIRECT:    i32 = 0o0040000;
pub const O_LARGEFILE: i32 = 0o0100000;
pub const O_DIRECTORY: i32 = 0o0200000;
pub const O_NOFOLLOW:  i32 = 0o0400000;
pub const O_NOATIME:   i32 = 0o1000000;
pub const O_CLOEXEC:   i32 = 0o2000000;
pub const O_SYNC:      i32 = 0o4000000;
pub const O_PATH:      i32 = 0o10000000;
pub const O_TMPFILE:   i32 = 0o20000000;
pub const O_NDELAY:    i32 = O_NONBLOCK;

pub const S_IRWXU: mode_t = 0o700;
pub const S_IRUSR: mode_t = 0o400;
pub const S_IWUSR: mode_t = 0o200;
pub const S_IXUSR: mode_t = 0o100;

pub const S_IRWXG: mode_t = 0o070;
pub const S_IRGRP: mode_t = 0o040;
pub const S_IWGRP: mode_t = 0o020;
pub const S_IXGRP: mode_t = 0o010;

pub const S_IRWXO: mode_t = 0o007;
pub const S_IROTH: mode_t = 0o004;
pub const S_IWOTH: mode_t = 0o002;
pub const S_IXOTH: mode_t = 0o001;

pub const SEEK_SET:  i32 = 0;
pub const SEEK_CUR:  i32 = 1;
pub const SEEK_END:  i32 = 2;
pub const SEEK_DATA: i32 = 3;
pub const SEEK_HOLE: i32 = 4;
pub const SEEK_MAX:  i32 = SEEK_HOLE;

// AUX Vector types
pub const AT_NULL:              u64 = 0;
pub const AT_IGNORE:            u64 = 1;
pub const AT_EXECFD:            u64 = 2;
pub const AT_PHDR:              u64 = 3;
pub const AT_PHENT:             u64 = 4;
pub const AT_PHNUM:             u64 = 5;
pub const AT_PAGESZ:            u64 = 6;
pub const AT_BASE:              u64 = 7;
pub const AT_FLAGS:             u64 = 8;
pub const AT_ENTRY:             u64 = 9;
pub const AT_NOTELF:            u64 = 10;
pub const AT_UID:               u64 = 11;
pub const AT_EUID:              u64 = 12;
pub const AT_GID:               u64 = 13;
pub const AT_EGID:              u64 = 14;
pub const AT_PLATFORM:          u64 = 15;
pub const AT_HWCAP:             u64 = 16;
pub const AT_CLKTCK:            u64 = 17;
pub const AT_SECURE:            u64 = 23;
pub const AT_BASE_PLATFORM:     u64 = 24;
pub const AT_RANDOM:            u64 = 25;
pub const AT_HWCAP2:            u64 = 26;
pub const AT_RSEQ_FEATURE_SIZE: u64 = 27;
pub const AT_RSEQ_ALIGN:        u64 = 28;
pub const AT_EXECFN:            u64 = 31;
pub const AT_SYSINFO:           u64 = 32;
pub const AT_SYSINFO_EHDR:      u64 = 33;
pub const AT_MINSIGSTKSZ:       u64 = 51;

// Clock ids
pub const CLOCK_REALTIME:           clockid_t = 0;
pub const CLOCK_MONOTONIC:          clockid_t = 1;
pub const CLOCK_PROCESS_CPUTIME_ID: clockid_t = 2;
pub const CLOCK_THREAD_CPUTIME_ID:  clockid_t = 3;
pub const CLOCK_MONOTONIC_RAW:      clockid_t = 4;
pub const CLOCK_REALTIME_COARSE:    clockid_t = 5;
pub const CLOCK_MONOTONIC_COARSE:   clockid_t = 6;
pub const CLOCK_BOOTTIME:           clockid_t = 7;
pub const CLOCK_REALTIME_ALARM:     clockid_t = 8;
pub const CLOCK_BOOTTIME_ALARM:     clockid_t = 9;

