use crate::elf::*;

pub(crate) struct Ldso {
    //exe: Memory,
    ldso: Memory,
}

impl Ldso {
    pub(crate) unsafe fn new(ldso: u64) -> Self {
        Self {
            //exe: Memory::new(exe),
            ldso: Memory::new(ldso),
        }
    }

    pub(crate) unsafe fn relocate_ldso(&self) {
        if let Some(slice) = self.ldso.rela() {
            for r in slice.into_iter() {
                let p = self.ldso.offset(r.r_offset) as *mut u64;
                let v = self.ldso.offset((elf_r_sym(r.r_info) as i64 + r.r_addend) as u64);
                p.write(v);
            }
        }
    }

    //pub(crate) unsafe fn relocate_exe(&self) {
    //    if let Some(slice) = self.exe.rela() {
    //        for r in slice.into_iter() {
    //            let p = self.exe.offset(r.r_offset) as *mut i64;
    //            let n = self.exe.get_sym_name(elf_r_sym(r.r_info) as usize).unwrap();
    //            let v = self.vdso.get_sym_addr(n).unwrap() as i64;
    //            p.write(v);
    //        }
    //    }
    //}
}
