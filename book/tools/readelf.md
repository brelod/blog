# readelf

```rust
// ==============================================================================
// Elf file
// ==============================================================================
#[derive(Debug, Clone)]
pub struct File<'a>{
    pub ehdr: &'a Ehdr,
    pub phdrs: &'a [Phdr],
    pub shdrs: &'a [Shdr],
}

impl<'a> File<'a> {
    pub unsafe fn from_slice(buf: &'a [u8]) -> Self {
        Self::from_ptr(buf.as_ptr())
    }

    pub unsafe fn from_ptr(p: *const u8) -> Self {
        let ehdr = &*(p as *const Ehdr);
        let phdrs = core::slice::from_raw_parts(
            p.offset(ehdr.e_phoff as isize) as *const Phdr,
            ehdr.e_phnum as usize
        );
        let shdrs = core::slice::from_raw_parts(
            p.offset(ehdr.e_shoff as isize) as *const Shdr,
            ehdr.e_shnum as usize
        );
        Self { ehdr, phdrs, shdrs }
    }

    pub unsafe fn shstr(&self, sh_name: u32) -> &str {
        let h = &self.shdrs[self.ehdr.e_shstrndx as usize];
        let p = (self.ehdr as *const _ as *const i8)
            .add(h.sh_offset as usize)
            .add(sh_name as usize);
        CStr::from_ptr(p).to_str().unwrap()
    }

    pub unsafe fn strtab(&self, st_name: u32) -> &str {
        for h in self.shdrs.into_iter() {
            if h.sh_type == SHT::SHT_STRTAB as u32 {
                let p = (self.ehdr as *const _ as *const i8)
                    .add(h.sh_offset as usize)
                    .add(st_name as usize);
                return CStr::from_ptr(p).to_str().unwrap();
            }
        }
        panic!("Missing strtab");
    }

    pub fn symtab(&self) -> &[Sym] {
        for h in self.shdrs.into_iter() {
            if h.sh_type == SHT::SHT_SYMTAB as u32 {
                return unsafe {
                    let p = (self.ehdr as *const _ as *const i8)
                        .add(h.sh_offset as usize) as *const Sym;
                    core::slice::from_raw_parts(p, (h.sh_size / h.sh_entsize) as usize)
                };
            }
        }

        &[]
    }

    pub fn dynsym(&self) -> &[Sym] {
        for h in self.shdrs.into_iter() {
            if h.sh_type == SHT::SHT_DYNSYM as u32 {
                return unsafe {
                    let p = (self.ehdr as *const _ as *const i8)
                        .add(h.sh_offset as usize) as *const Sym;
                    core::slice::from_raw_parts(p, (h.sh_size / h.sh_entsize) as usize)
                };
            }
        }

        &[]
    }

    pub fn dynamic(&self) -> &[Dyn] {
        for h in self.shdrs.into_iter() {
            if h.sh_type == SHT::SHT_DYNAMIC as u32 {
                return unsafe {
                    let p = (self.ehdr as *const _ as *const i8)
                        .add(h.sh_offset as usize) as *const Dyn;
                    core::slice::from_raw_parts(p, (h.sh_size / h.sh_entsize) as usize)
                };
            }
        }

        &[]
    }

    pub fn dump_phdrs(&self) {
        crate::println!("Program headers:");
        //crate::println!("{:?}", ""); // NOTE: Without this it will segfault in the for loop...
        crate::println!("  {:<3} {:<12} {:<10} {:<18} {:<18} {:<10} {:<10} {:<3} {:<10}", 
            "Idx",
            "Type", 
            "Offset",
            "VirtAddr",
            "PhysAddr",
            "FileSize",
            "MemSize",
            "Flg",
            "Align"
        );

        //crate::println!("{:?}", self.phdrs);
        for (idx, h) in self.phdrs.into_iter().enumerate() {
            crate::println!("  {:<3?} {:<12} 0x{:0>8x?} 0x{:0>16x?} 0x{:0>16x?} 0x{:0>8x?} 0x{:0>8x?} {:<3} 0x{:0>8x?}",
                idx,
                h.p_type().as_str(),
                h.p_offset,
                h.p_vaddr,
                h.p_paddr,
                h.p_filesz,
                h.p_memsz,
                h.flags(),
                h.p_align
            );
        }

        crate::println!("");
    }

    pub fn dump_shdrs(&self) {
        //crate::println!("{:?}", ""); // NOTE: Without this it will segfault in the for loop...
        crate::println!("Section headers:");
        crate::println!("  {:<3} {:<13} {:<18} {:<10} {:<10} {:<3} {:<3} {:<3} {:<3} {:<3} {}", 
            "Idx",
            "Type", 
            "Address",
            "Offset",
            "Size",
            "ENS",
            "FLG",
            "LNK",
            "INF",
            "ALI",
            "Name"
        );

        //crate::println!("{:?}", self.phdrs);
        for (idx, h) in self.shdrs.into_iter().enumerate() {
            crate::println!("  {:<3} {:<13} 0x{:0>16x} 0x{:0>8x} 0x{:0>8x} {:<3} {:<3} {:<3} {:<3} {:<3} {}", 
                idx,
                h.sh_type().as_str(),
                h.sh_addr,
                h.sh_offset,
                h.sh_size,
                h.sh_entsize,
                h.sh_flags,
                h.sh_link,
                h.sh_info,
                h.sh_addralign,
                unsafe { self.shstr(h.sh_name) }
            );
        }

        crate::println!("");
    }

    fn dump_symbols(&self, symbols: &[Sym]) {
        crate::println!("  {:<3} {:<8} {:<18} {:<10} {:<6} {:<10} {:<5} {}", 
            "Idx",
            "Type",
            "Address", 
            "Size",
            "Bind",
            "Visibility",
            "Shndx",
            "Name"
        );

        //crate::println!("{:?}", self.phdrs);
        for (idx, s) in symbols.into_iter().enumerate() {
            crate::println!("  {:<3} {:<8} 0x{:0>16x} 0x{:0>8x} {:<6} {:<10} {:<5} {}", 
                idx,
                s.st_type().as_str(),
                s.st_value,
                s.st_size,
                s.st_bind().as_str(),
                s.st_visibility().as_str(),
                s.st_shndx,
                unsafe { self.strtab(s.st_name) }
            );
        }
    }

    pub fn dump_symtab(&self) {
        //crate::println!("{:?}", ""); // NOTE: Without this it will segfault in the for loop...
        let symtab = self.symtab();

        crate::println!("Static symbols: {:?}", symtab.len());
        if symtab.len() > 0 {
            self.dump_symbols(symtab);
        }

        crate::println!("");
    }

    pub fn dump_dynsym(&self) {
        //crate::println!("{:?}", ""); // NOTE: Without this it will segfault in the for loop...
        let dynsym = self.dynsym();
        crate::println!("Dynamic symbols: {:?}", dynsym.len());
        if dynsym.len() > 0 {
            self.dump_symbols(dynsym);
        }

        crate::println!("");
    }

    pub fn dump_dynamic(&self) {
        let dynamic = self.dynamic();
        crate::println!("Dynamic reloaction: {:?}", dynamic.len());

        if dynamic.len() == 0 {
            return;
        }

        crate::println!("  {:<3} {:<15} {}", "Idx", "Type", "Value");

        //crate::println!("{:?}", self.phdrs);
        for (idx, s) in dynamic.into_iter().enumerate() {
            match s.d_tag() {
                DT::DT_SONAME | DT::DT_NEEDED => {
                    crate::println!("  {:<3} {:<15} {}",
                        idx,
                        s.d_tag().as_str(),
                        unsafe { self.strtab(s.d_val as u32) }
                    );
                }
                _ => {
                    crate::println!("  {:<3} {:<15} 0x{:0>16x}",
                        idx,
                        s.d_tag().as_str(),
                        s.d_val
                    );
                }
            }

            if let DT::DT_NULL = s.d_tag() {
                break
            }
        }

        crate::println!("");
    }
}
```

