#![no_std]
#![no_main]
#![feature(linkage)]
#![allow(unused_imports, unused_variables, unused_assignments, dead_code)]

#[macro_use]
extern crate linux;
use linux::ldso::*;
use linux::types::*;


fn readelf(path: &str) {
    use linux::syscall::*;
    use linux::constants::*;
    let fd = open(path, O_RDONLY, 0).unwrap();
    let mut stat = stat64::default();
    fstat(fd, &mut stat).unwrap();
    unsafe {
        let p = mmap(
            core::ptr::null_mut(),
            stat.st_size as usize,
            PROT_READ,
            MAP_PRIVATE,
            fd, 0
        ).unwrap();
        let elf = linux::elf::File::from_ptr(p);
        elf.dump_dynsym();
        elf.dump_rel();
        elf.dump_rela();
    }
}

fn link() {
    let mut entry = 0x0 as *const u8;

    for aux in linux::env::auxv() {
        match aux {
            AT::AT_ENTRY(ptr) => { entry = ptr; break; }
            other => { /* ignore */ }
        }
    }


    let _start: extern "C" fn() -> ! = unsafe { core::mem::transmute(entry) };
    _start();
}


#[no_mangle]
fn main() -> u8 { 
    for aux in linux::env::auxv() {
        if let AT::AT_ENTRY(entry) = aux {
            unsafe { 
                core::arch::asm!(
                    "jmp {}", 
                    in(reg) entry,
                    options(nostack, noreturn),
                );
            }
        }
    }

    unreachable!()
}

