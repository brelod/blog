#![no_std]
#![no_main]
#![feature(lang_items)]
#![allow(internal_features)]

use core::mem::size_of;

pub mod elf;
pub mod ffi;
pub mod auxv;
pub mod ldso;

use ldso::Ldso;
use auxv::auxv_t;

unsafe fn auxv(rsp: usize) -> *const auxv_t {
    let argc = *(rsp as *const usize);
    //let argv = (rsp + 8) as *mut *const i8;
    let envp = (rsp + 8 + 8 + argc * 8) as *mut *const i8;

    let mut p = envp;
    while !(*p).is_null() { 
        p = p.offset(1);
    }
    p.offset(1) as *mut auxv_t
}

#[no_mangle]
unsafe fn main(rsp: usize) -> u8 {
    //let mut exe = 0x0;
    let mut ldso = None;
    let mut entry = None;
    let mut auxv = auxv(rsp);

    loop {
        match (*auxv).key {
            auxv::AT_NULL => { break; }
            auxv::AT_BASE => { ldso = Some((*auxv).val); }
            //auxv::AT_PHDR => { exe = (*auxv).val - size_of::<elf::Ehdr>() as u64; }
            auxv::AT_ENTRY => { entry = Some((*auxv).val); }
            _ => { /* ignore */ }
        }
        auxv = auxv.add(1);
    }

    if let Some(ldso) = ldso {
        Ldso::new(ldso).relocate_ldso();
    }

    if let Some(entry) = entry {
        core::arch::asm!(
            "jmp {}", 
            in(reg) entry,
            options(nostack, noreturn),
        );
    }

    255
}

//#[lang = "eh_personality"]
//fn rust_eh_personality() {}

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
