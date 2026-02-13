#![no_std]
#![no_main]
#![feature(lang_items)]
#![allow(internal_features)]
#![crate_name = "linux"]

use core::fmt::Write;
use core::sync::atomic::{AtomicPtr, Ordering};

pub mod io;
pub mod elf;
pub mod env;
pub mod ffi;
pub mod vdso;
pub mod ldso;
pub mod error;
pub mod types;
pub mod limits;
pub mod syscall;
#[rustfmt::skip]
pub mod constants;

use vdso::Vdso;
use elf::*;
use types::*;
use constants::*;

pub use error::Error;
pub use error::Result;

pub(crate) static ARGV: AtomicPtr<*const i8> = AtomicPtr::new(core::ptr::null_mut());
pub(crate) static ENVP: AtomicPtr<*const i8> = AtomicPtr::new(core::ptr::null_mut());
pub(crate) static AUXV: AtomicPtr<auxv_t> = AtomicPtr::new(core::ptr::null_mut());

extern "C" {
    fn main() -> u8;
}

#[no_mangle]
fn _start() -> ! {
    unsafe {
        core::arch::asm!(
            "pop rax",      // compensate force-frame-pointers=yes
            "xor rbp,rbp",
            "and rsp,-16",
            "mov rdi,rsp",
            "call __rust_main",
            "mov rdi,rax",
            "mov rax,0x3c",
            "syscall",
            options(nostack, noreturn),
        )
    }
}

/* ANCHOR: __rust_main */
#[no_mangle]
unsafe fn __rust_main(rsp: *const u8) -> u8 {
    parse_stack(rsp);
    //let ldso = ldso::Ldso::new();
    //ldso.relocate_ldso();
    //ldso.relocate_exe();
    main()
}
/* ANCHOR_END: __rust_main */

unsafe fn parse_stack(rsp: *const u8) {
    let argc = *(rsp as *const isize);
    let argv = rsp.offset(8) as *mut *const i8;
    let envp = rsp.offset(8 + 8 + argc * 8) as *mut *const i8;

    let mut p = envp;
    while !(*p).is_null() { 
        p = p.offset(1);
    }
    let auxv = p.offset(1) as *mut auxv_t;

    ARGV.store(argv, Ordering::Relaxed);
    ENVP.store(envp, Ordering::Relaxed);
    AUXV.store(auxv, Ordering::Relaxed);
}



#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    let _ = write!(crate::io::stderr(), "{}\n", info);
    syscall::exit(255);
}

#[lang = "eh_personality"]
fn rust_eh_personality() {}

#[no_mangle]
fn cstr(src: &[u8], dst: &mut [u8]) -> Result<()> {
    if src.len() >= dst.len() {
        return Err(Error::ENAMETOOLONG);
    }

    for idx in 0..src.len() {
        dst[idx] = src[idx];
    }

    dst[src.len()] = 0;
    Ok(())
}
