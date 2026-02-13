use core::sync::atomic::{AtomicPtr, Ordering};

use crate::elf::*;
use crate::types::*;
use crate::syscall::*;
use crate::constants::*;

pub(crate) struct Ldso {
    exe: Memory,
    vdso: Memory,
    ldso: Memory,
}

impl Ldso {
    pub(crate) unsafe fn new() -> Self {
        let mut exe = 0x0;
        let mut vdso = 0x0;
        let mut ldso = 0x0;

        use crate::constants as C;
        let mut p = crate::AUXV.load(Ordering::Relaxed);
        loop {
            match (*p).key {
                C::AT_NULL => { break; }
                C::AT_BASE => { ldso = (*p).val; }
                C::AT_SYSINFO_EHDR => { vdso = (*p).val; }
                C::AT_PHDR => { exe = (*p).val - core::mem::size_of::<Ehdr>() as u64; }
                _ => { /* ignore */ }
            }
            p = p.add(1);
        }

        let exe = Memory::new(exe);
        let vdso = Memory::new(vdso);
        let ldso = Memory::new(ldso);

        Self { exe, vdso, ldso }
    }


    pub(crate) unsafe fn relocate_ldso(&self) {
        if let Some(slice) = self.ldso.rela() {
            for r in slice.into_iter() {
                let p = self.ldso.offset(r.r_offset) as *mut u64;
                let v = self.ldso.offset((r.r_sym() as i64 + r.r_addend) as u64);
                p.write(v);
            }
        }
        crate::println!("Relocate ldso done");
    }

    pub(crate) unsafe fn relocate_exe(&self) {
        if let Some(slice) = self.exe.rela() {
            for r in slice.into_iter() {
                let p = self.exe.offset(r.r_offset) as *mut i64;
                let n = self.exe.get_sym_name(r.r_sym() as usize).unwrap();
                let v = self.vdso.get_sym_addr(n).unwrap() as i64;
                p.write(v);
            }
        }
        crate::println!("Relocate exe done");
    }
}
