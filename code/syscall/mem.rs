#![doc = include_str!(".mem/intro.md")]

use crate::cstr;
use crate::types::*;
use crate::constants::*; // For docs links
use crate::error::{Error, Result, result};
use crate::error::Error::*;

// ==============================================================================
// Huge page encoding (include/uapi/asm-generic/hugetlb_encode.h)
// ==============================================================================
pub const HUGETLB_FLAG_ENCODE_SHIFT: i32 = 26;
pub const HUGETLB_FLAG_ENCODE_MASK:  i32 = 0x3f;

pub const HUGETLB_FLAG_ENCODE_16KB:  i32 = 14 << HUGETLB_FLAG_ENCODE_SHIFT;
pub const HUGETLB_FLAG_ENCODE_64KB:  i32 = 16 << HUGETLB_FLAG_ENCODE_SHIFT;
pub const HUGETLB_FLAG_ENCODE_512KB: i32 = 19 << HUGETLB_FLAG_ENCODE_SHIFT;
pub const HUGETLB_FLAG_ENCODE_1MB:   i32 = 20 << HUGETLB_FLAG_ENCODE_SHIFT;
pub const HUGETLB_FLAG_ENCODE_2MB:   i32 = 21 << HUGETLB_FLAG_ENCODE_SHIFT;
pub const HUGETLB_FLAG_ENCODE_8MB:   i32 = 23 << HUGETLB_FLAG_ENCODE_SHIFT;
pub const HUGETLB_FLAG_ENCODE_16MB:  i32 = 24 << HUGETLB_FLAG_ENCODE_SHIFT;
pub const HUGETLB_FLAG_ENCODE_32MB:  i32 = 25 << HUGETLB_FLAG_ENCODE_SHIFT;
pub const HUGETLB_FLAG_ENCODE_256MB: i32 = 28 << HUGETLB_FLAG_ENCODE_SHIFT;
pub const HUGETLB_FLAG_ENCODE_512MB: i32 = 29 << HUGETLB_FLAG_ENCODE_SHIFT;
pub const HUGETLB_FLAG_ENCODE_1GB:   i32 = 30 << HUGETLB_FLAG_ENCODE_SHIFT;
pub const HUGETLB_FLAG_ENCODE_2GB:   i32 = 31 << HUGETLB_FLAG_ENCODE_SHIFT;
pub const HUGETLB_FLAG_ENCODE_16GB:  i32 = 34 << HUGETLB_FLAG_ENCODE_SHIFT;

// ==============================================================================
// # Memory protection (include/uapi/asm-generic/mman-common.h)
// ==============================================================================
pub const PROT_NONE:      i32 = 0x00000000;
pub const PROT_READ:      i32 = 0x00000001;
pub const PROT_WRITE:     i32 = 0x00000002;
pub const PROT_EXEC:      i32 = 0x00000004;
pub const PROT_SEM:       i32 = 0x00000008;
pub const PROT_GROWSDOWN: i32 = 0x01000000;
pub const PROT_GROWSUP:   i32 = 0x02000000;


// ==============================================================================
// Memory behaviour (uapi/asm-generic/mman-common.h and mmap.h)
// ==============================================================================
// Ignored flags:
// - MAP_EXECUTABLE;
// - MAP_FILE;
// - MAP_DENYWRITE;
// - MAP_EXECUTABLE;
// - MAP_LOCKED: i32;
pub const MAP_32BIT:           i32 = 0x0000000;
pub const MAP_SHARED:          i32 = 0x0000001;
pub const MAP_PRIVATE:         i32 = 0x0000002;
pub const MAP_SHARED_VALIDATE: i32 = 0x0000003;
pub const MAP_TYPE:            i32 = 0x000000f;
pub const MAP_FIXED:           i32 = 0x0000010;
pub const MAP_ANONYMOUS:       i32 = 0x0000020;
pub const MAP_GROWSDOWN:       i32 = 0x0000100;
pub const MAP_NORESERVE:       i32 = 0x0004000;
pub const MAP_POPULATE:        i32 = 0x0008000;
pub const MAP_NONBLOCK:        i32 = 0x0010000;
pub const MAP_STACK:           i32 = 0x0020000;
pub const MAP_HUGETLB:         i32 = 0x0040000;
pub const MAP_SYNC:            i32 = 0x0080000;
pub const MAP_FIXED_NOREPLACE: i32 = 0x0100000;
pub const MAP_UNINITIALIZED:   i32 = 0x4000000;

pub const MAP_HUGE_SHIFT: i32 = HUGETLB_FLAG_ENCODE_SHIFT;
pub const MAP_HUGE_MASK:  i32 = HUGETLB_FLAG_ENCODE_MASK;

pub const MAP_HUGE_16KB:  i32 = HUGETLB_FLAG_ENCODE_16KB;
pub const MAP_HUGE_64KB:  i32 = HUGETLB_FLAG_ENCODE_64KB;
pub const MAP_HUGE_512KB: i32 = HUGETLB_FLAG_ENCODE_512KB;
pub const MAP_HUGE_1MB:   i32 = HUGETLB_FLAG_ENCODE_1MB;
pub const MAP_HUGE_2MB:   i32 = HUGETLB_FLAG_ENCODE_2MB;
pub const MAP_HUGE_8MB:   i32 = HUGETLB_FLAG_ENCODE_8MB;
pub const MAP_HUGE_16MB:  i32 = HUGETLB_FLAG_ENCODE_16MB;
pub const MAP_HUGE_32MB:  i32 = HUGETLB_FLAG_ENCODE_32MB;
pub const MAP_HUGE_256MB: i32 = HUGETLB_FLAG_ENCODE_256MB;
pub const MAP_HUGE_512MB: i32 = HUGETLB_FLAG_ENCODE_512MB;
pub const MAP_HUGE_1GB:   i32 = HUGETLB_FLAG_ENCODE_1GB;
pub const MAP_HUGE_2GB:   i32 = HUGETLB_FLAG_ENCODE_2GB;
pub const MAP_HUGE_16GB:  i32 = HUGETLB_FLAG_ENCODE_16GB;


// Location: include/uapi/linux/mman.h
pub const MREMAP_MAYMOVE:   i32 = 1;
pub const MREMAP_FIXED:     i32 = 2;
pub const MREMAP_DONTUNMAP: i32 = 4;



#[no_mangle]
#[doc = include_str!(".mem/mmap.md")]
pub unsafe fn mmap(addr: *mut u8, len: usize, prot: i32, flags: i32, fd: u32, off: off_t) -> Result<*mut u8> {
    result(unsafe { syscall!(super::SYS_MMAP, addr, len, prot, flags, fd, off) }).map(|n| n as *mut u8)
}

#[no_mangle]
#[doc = include_str!(".mem/mprotect.md")]
pub unsafe fn mprotect(addr: *mut u8, len: usize, prot: i32) -> Result<()> {
    result(unsafe { syscall!(super::SYS_MPROTECT, addr, len, prot) }).map(|_| ())
}

#[no_mangle]
#[doc = include_str!(".mem/mremap.md")]
pub unsafe fn mremap(old_addr: *mut u8, old_len: usize, new_len: usize, flags: i32, new_addr: *mut u8) -> Result<*mut u8> {
    result(unsafe { syscall!(super::SYS_MREMAP, old_addr, old_len, new_len, flags, new_addr) }).map(|n| n as *mut u8)
}

#[no_mangle]
#[doc = include_str!(".mem/munmap.md")]
pub unsafe fn munmap(addr: *mut u8, len: usize) -> Result<()> {
    result(unsafe { syscall!(super::SYS_MUNMAP, addr, len) }).map(|_| ())
}

#[no_mangle]
#[doc = include_str!(".mem/brk.md")]
pub fn brk(addr: isize) -> isize {
    unsafe { syscall!(super::SYS_BRK, addr) }
}

#[no_mangle]
pub fn msync() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_MSYNC) })
}

#[no_mangle]
pub fn mincore() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_MINCORE) })
}

#[no_mangle]
pub fn madvise() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_MADVISE) })
}

#[no_mangle]
pub fn mlock() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_MLOCK) })
}

#[no_mangle]
pub fn munlock() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_MUNLOCK) })
}

#[no_mangle]
pub fn mlockall() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_MLOCKALL) })
}

#[no_mangle]
pub fn munlockall() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_MUNLOCKALL) })
}
#[no_mangle]
pub fn remap_file_pages() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_REMAP_FILE_PAGES) })
}
#[no_mangle]
pub fn mbind() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_MBIND) })
}

#[no_mangle]
pub fn set_mempolicy() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_SET_MEMPOLICY) })
}

#[no_mangle]
pub fn get_mempolicy() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_GET_MEMPOLICY) })
}
#[no_mangle]
pub fn add_key() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_ADD_KEY) })
}

#[no_mangle]
pub fn request_key() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_REQUEST_KEY) })
}

#[no_mangle]
pub fn keyctl() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_KEYCTL) })
}
#[no_mangle]
pub fn migrate_pages() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_MIGRATE_PAGES) })
}
#[no_mangle]
pub fn move_pages() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_MOVE_PAGES) })
}

#[no_mangle]
pub fn process_vm_readv() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_PROCESS_VM_READV) })
}

#[no_mangle]
pub fn process_vm_writev() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_PROCESS_VM_WRITEV) })
}
#[no_mangle]
pub fn memfd_create() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_MEMFD_CREATE) })
}

#[no_mangle]
pub fn memfd_secret() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_MEMFD_SECRET) })
}
#[no_mangle]
pub fn membarrier() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_MEMBARRIER) })
}

#[no_mangle]
pub fn mlock2() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_MLOCK2) })
}
#[no_mangle]
pub fn pkey_mprotect() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_PKEY_MPROTECT) })
}

#[no_mangle]
pub fn pkey_alloc() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_PKEY_ALLOC) })
}

#[no_mangle]
pub fn pkey_free() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_PKEY_FREE) })
}
#[no_mangle]
pub fn process_madvise() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_PROCESS_MADVISE) })
}
#[no_mangle]
pub fn process_mrelease() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_PROCESS_MRELEASE) })
}

#[no_mangle]
pub fn set_mempolicy_home_node() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_SET_MEMPOLICY_HOME_NODE) })
}

#[no_mangle]
pub fn cachestat() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_CACHESTAT) })
}
#[no_mangle]
pub fn map_shadow_stack() -> Result<usize> {
    result(unsafe { syscall!(super::SYS_MAP_SHADOW_STACK) })
}

