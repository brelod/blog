#![allow(unused)]
#![allow(non_camel_case_types)]
use core::ffi::CStr;

// ==============================================================================
// Elf header
// ==============================================================================
pub const PN_XNUM: u16 = 0xffff;
pub const EI_NIDENT:  usize = 16;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Ehdr {
    pub e_ident: [u8;EI_NIDENT],
    pub e_type: u16,
    pub e_machine: u16,
    pub e_version: u32,
    pub e_entry: u64,
    pub e_phoff: u64,
    pub e_shoff: u64,
    pub e_flags: u32,
    pub e_ehsize: u16,
    pub e_phentsize: u16,
    pub e_phnum: u16,
    pub e_shentsize: u16,
    pub e_shnum: u16,
    pub e_shstrndx: u16,
}

// ==============================================================================
// Program headers
// ==============================================================================
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Phdr {
    pub p_type: u32,
    pub p_flags: u32,
    pub p_offset: u64,
    pub p_vaddr: u64,
    pub p_paddr: u64,
    pub p_filesz: u64,
    pub p_memsz: u64,
    pub p_align: u64,
}

const PT_LOOS:   u32 = 0x60000000;
const PT_HIOS:   u32 = 0x6fffffff;
const PT_LOPROC: u32 = 0x70000000;
const PT_HIPROC: u32 = 0x7fffffff;

pub const PT_NULL:         u32 = 0;
pub const PT_LOAD:         u32 = 1;
pub const PT_DYNAMIC:      u32 = 2;
pub const PT_INTERP:       u32 = 3;
pub const PT_NOTE:         u32 = 4;
pub const PT_SHLIB:        u32 = 5;
pub const PT_PHDR:         u32 = 6;
pub const PT_TLS:          u32 = 7;
pub const PT_GNU_EH_FRAME: u32 = PT_LOOS + 0x474e550;
pub const PT_GNU_STACK:    u32 = PT_LOOS + 0x474e551;
pub const PT_GNU_RELRO:    u32 = PT_LOOS + 0x474e552;
pub const PT_GNU_PROPERTY: u32 = PT_LOOS + 0x474e553;

// Program segment permissions (Phdr.p_flags)
pub const PF_R: u32 = 0x4;
pub const PF_W: u32 = 0x2;
pub const PF_X: u32 = 0x1;


// ==============================================================================
// Section headers
// ==============================================================================
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Shdr {
    pub sh_name: u32,
    pub sh_type: u32,
    pub sh_flags: u64,
    pub sh_addr: u64,
    pub sh_offset: u64,
    pub sh_size: u64,
    pub sh_link: u32,
    pub sh_info: u32,
    pub sh_addralign: u64,
    pub sh_entsize: u64,
}

// sh_type
pub const SHT_NULL:           u32 = 0;
pub const SHT_PROGBITS:       u32 = 1;
pub const SHT_SYMTAB:         u32 = 2;
pub const SHT_STRTAB:         u32 = 3;
pub const SHT_RELA:           u32 = 4;
pub const SHT_HASH:           u32 = 5;
pub const SHT_DYNAMIC:        u32 = 6;
pub const SHT_NOTE:           u32 = 7;
pub const SHT_NOBITS:         u32 = 8;
pub const SHT_REL:            u32 = 9;
pub const SHT_SHLIB:          u32 = 10;
pub const SHT_DYNSYM:         u32 = 11;
pub const SHT_NUM:            u32 = 12;
pub const SHT_INIT_ARRAY:     u32 = 14;
pub const SHT_FINI_ARRAY:     u32 = 15;
pub const SHT_PREINIT_ARRAY:  u32 = 16;
pub const SHT_GROUP:          u32 = 17;
pub const SHT_SYMTAB_SHNDX:   u32 = 18;
pub const SHT_GNU_ATTRIBUTES: u32 = 0x6ffffff5;
pub const SHT_GNU_HASH:       u32 = 0x6ffffff6;
pub const SHT_GNU_LIBLIST:    u32 = 0x6ffffff7;
pub const SHT_CHECKSUM:       u32 = 0x6ffffff8;
pub const SHT_GNU_VERDEF:     u32 = 0x6ffffffd;
pub const SHT_GNU_VERNEED:    u32 = 0x6ffffffe;
pub const SHT_GNU_VERSYM:     u32 = 0x6fffffff;

// sh_flags
pub const SHF_WRITE:          u64 = 0x00000001;
pub const SHF_ALLOC:          u64 = 0x00000002;
pub const SHF_EXECINSTR:      u64 = 0x00000004;
pub const SHF_RELA_LIVEPATCH: u64 = 0x00100000;
pub const SHF_RO_AFTER_INIT:  u64 = 0x00200000;
pub const SHF_MASKPROC:       u64 = 0xf0000000;

// Special section indexes
pub const SHN_UNDEF:     usize = 0x0000;
pub const SHN_LORESERVE: usize = 0xff00;
pub const SHN_LOPROC:    usize = 0xff00;
pub const SHN_HIPROC:    usize = 0xff1f;
pub const SHN_LIVEPATCH: usize = 0xff20;
pub const SHN_ABS:       usize = 0xfff1;
pub const SHN_COMMON:    usize = 0xfff2;
pub const SHN_HIRESERVE: usize = 0xffff;


// ==============================================================================
// Symbol table
// ==============================================================================
/// Symbol table entry
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Sym {
    pub st_name: u32,
    pub st_info: u8,
    pub st_other: u8,
    pub st_shndx: u16,
    pub st_value: u64,
    pub st_size: u64,
}

pub const STT_NOTYPE:    u8 = 0;
pub const STT_OBJECT:    u8 = 1;
pub const STT_FUNC:      u8 = 2;
pub const STT_SECTION:   u8 = 3;
pub const STT_FILE:      u8 = 4;
pub const STT_COMMON:    u8 = 5;
pub const STT_TLS:       u8 = 6;

pub const STB_LOCAL:     u8 = 0;
pub const STB_GLOBAL:    u8 = 1;
pub const STB_WEAK:      u8 = 2;

pub const STV_DEFAULT:   u8 = 0;
pub const STV_INTERNAL:  u8 = 1;
pub const STV_HIDDEN:    u8 = 2;
pub const STV_PROTECTED: u8 = 3;

// ==============================================================================
// Dynamic linking table
// ==============================================================================
/// Dynamic linking table entry
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Dyn {
    pub d_tag: i64,
    pub d_val: u64, // it's a union in the c_code but both variant is u64 (d_val, d_ptr)
}


const DT_VALRNGLO:  i64 = 0x6ffffd00;
const DT_VALRNGHI:  i64 = 0x6ffffdff;
const DT_ADDRRNGLO: i64 = 0x6ffffe00;
const DT_ADDRRNGHI: i64 = 0x6ffffeff;
const OLD_DT_LOOS:  i64 = 0x60000000;
const DT_LOOS:      i64 = 0x6000000d;
const DT_HIOS:      i64 = 0x6ffff000;
const DT_LOPROC:    i64 = 0x70000000;
const DT_HIPROC:    i64 = 0x7fffffff;

pub const DT_NULL:            i64 = 0;
pub const DT_NEEDED:          i64 = 1;
pub const DT_PLTRELSZ:        i64 = 2;
pub const DT_PLTGOT:          i64 = 3;
pub const DT_HASH:            i64 = 4;
pub const DT_STRTAB:          i64 = 5;
pub const DT_SYMTAB:          i64 = 6;
pub const DT_RELA:            i64 = 7;
pub const DT_RELASZ:          i64 = 8;
pub const DT_RELAENT:         i64 = 9;
pub const DT_STRSZ:           i64 = 10;
pub const DT_SYMENT:          i64 = 11;
pub const DT_INIT:            i64 = 12;
pub const DT_FINI:            i64 = 13;
pub const DT_SONAME:          i64 = 14;
pub const DT_RPATH:           i64 = 15;
pub const DT_SYMBOLIC:        i64 = 16;
pub const DT_REL:             i64 = 17;
pub const DT_RELSZ:           i64 = 18;
pub const DT_RELENT:          i64 = 19;
pub const DT_PLTREL:          i64 = 20;
pub const DT_DEBUG:           i64 = 21;
pub const DT_TEXTREL:         i64 = 22;
pub const DT_JMPREL:          i64 = 23;
pub const DT_BIND_NOW:        i64 = 24;
pub const DT_INIT_ARRAY:      i64 = 25;
pub const DT_FINI_ARRAY:      i64 = 26;
pub const DT_INIT_ARRAYSZ:    i64 = 27;
pub const DT_FINI_ARRAYSZ:    i64 = 28;
pub const DT_RUNPATH:         i64 = 29;
pub const DT_FLAGS:           i64 = 30;
pub const DT_ENCODING:        i64 = 32;
pub const DT_PREINIT_ARRAYSZ: i64 = 33;
pub const DT_NUM:             i64 = 34;
pub const DT_GNU_HASH:        i64 = 0x6ffffef5;
pub const DT_VERSYM:          i64 = 0x6ffffff0;
pub const DT_RELACOUNT:       i64 = 0x6ffffff9;
pub const DT_RELCOUNT:        i64 = 0x6ffffffa;
pub const DT_FLAGS_1:         i64 = 0x6ffffffb;
pub const DT_VERDEF:          i64 = 0x6ffffffc;
pub const DT_VERDEFNUM:       i64 = 0x6ffffffd;
pub const DT_VERNEED:         i64 = 0x6ffffffe;
pub const DT_VERNEEDNUM:      i64 = 0x6fffffff;
pub const DT_UNKNOWN:         i64 = DT_HIPROC;

// ==============================================================================
// Relocation table
// ==============================================================================
pub fn elf_r_sym(r_info: u64) -> u64 { r_info >> 32 }
pub fn elf_r_type(r_info: u64) -> u64 { r_info & 0xffffffff }

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Rel {
    pub r_offset: u64,
    pub r_info: u64,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Rela {
    pub r_offset: u64,
    pub r_info: u64,
    pub r_addend: i64,
}

pub const R_X86_64_NONE:            u32 = 0;
pub const R_X86_64_64:              u32 = 1;
pub const R_X86_64_PC32:            u32 = 2;
pub const R_X86_64_GOT32:           u32 = 3;
pub const R_X86_64_PLT32:           u32 = 4;
pub const R_X86_64_COPY:            u32 = 5;
pub const R_X86_64_GLOB_DAT:        u32 = 6;
pub const R_X86_64_JUMP_SLOT:       u32 = 7;
pub const R_X86_64_RELATIVE:        u32 = 8;
pub const R_X86_64_GOTPCREL:        u32 = 9;
pub const R_X86_64_32:              u32 = 10;
pub const R_X86_64_32S:             u32 = 11;
pub const R_X86_64_16:              u32 = 12;
pub const R_X86_64_PC16:            u32 = 13;
pub const R_X86_64_8:               u32 = 14;
pub const R_X86_64_PC8:             u32 = 15;
pub const R_X86_64_DTPMOD64:        u32 = 16;
pub const R_X86_64_DTPOFF64:        u32 = 17;
pub const R_X86_64_TPOFF64:         u32 = 18;
pub const R_X86_64_TLSGD:           u32 = 19;
pub const R_X86_64_TLSLD:           u32 = 20;
pub const R_X86_64_DTPOFF32:        u32 = 21;
pub const R_X86_64_GOTTPOFF:        u32 = 22;
pub const R_X86_64_TPOFF32:         u32 = 23;
pub const R_X86_64_PC64:            u32 = 24;
pub const R_X86_64_GOTOFF64:        u32 = 25;
pub const R_X86_64_GOTPC32:         u32 = 26;
pub const R_X86_64_GOT64:           u32 = 27;
pub const R_X86_64_GOTPCREL64:      u32 = 28;
pub const R_X86_64_GOTPC64:         u32 = 29;
pub const R_X86_64_GOTPLT64:        u32 = 30;
pub const R_X86_64_PLTOFF64:        u32 = 31;
pub const R_X86_64_SIZE32:          u32 = 32;
pub const R_X86_64_SIZE64:          u32 = 33;
pub const R_X86_64_GOTPC32_TLSDESC: u32 = 34;
pub const R_X86_64_TLSDESC_CALL:    u32 = 35;
pub const R_X86_64_TLSDESC:         u32 = 36;
pub const R_X86_64_IRELATIVE:       u32 = 37;
pub const R_X86_64_RELATIVE64:      u32 = 38;
pub const R_X86_64_GOTPCRELX:       u32 = 41;
pub const R_X86_64_REX_GOTPCRELX:   u32 = 42;


// ==============================================================================
// Memory handler of an ELF file
// ==============================================================================
#[derive(Debug, Clone)]
pub struct Memory{
    base: u64,
}

impl Memory {
    pub fn new(base: u64) -> Self {
        Self { base }
    }

    pub unsafe fn offset(&self, offset: u64) -> u64 {
        self.base + offset
    }

    pub unsafe fn patch(&self, offset: u64, value: u64) {
        *(self.offset(offset) as *mut u64) = value;
    }

    pub unsafe fn phdrs(&self) -> &[Phdr] {
        let ehdr = &*(self.base as *const Ehdr);
        core::slice::from_raw_parts(
            (self.base + ehdr.e_phoff) as *const Phdr,
            ehdr.e_phnum as usize
        )
    }

    pub unsafe fn shdrs(&self) -> &[Shdr] {
        let ehdr = &*(self.base as *const Ehdr);
        core::slice::from_raw_parts(
            (self.base + ehdr.e_shoff) as *const Shdr,
            ehdr.e_shnum as usize
        )
    }

    pub unsafe fn pt_dynamic(&self) -> Option<&[Dyn]> {
        for h in self.phdrs().into_iter() {
            if h.p_type == PT_DYNAMIC as u32 {
                let p = (self.base + h.p_vaddr) as *const Dyn;
                return Some(core::slice::from_raw_parts(
                    p, h.p_memsz as usize / core::mem::size_of::<Dyn>()
                ));
            }
        }

        None
    }

    pub unsafe fn strtab(&self) -> Option<*const i8> {
        if let Some(slice) = self.pt_dynamic() {
            for h in slice.into_iter() {
                if h.d_tag == DT_STRTAB {
                    return Some((self.base + h.d_val) as *const i8);
                }
            }
        }

        None
    }

    pub unsafe fn dynsym(&self) -> Option<&[Sym]> {
        for h in self.shdrs().into_iter() {
            if h.sh_type == SHT_DYNSYM as u32 {
                return Some(core::slice::from_raw_parts(
                    (self.base + h.sh_offset) as *const Sym,
                    (h.sh_size / h.sh_entsize) as usize,
                ));
            }
        }

        None
    }

    #[inline(never)]
    pub unsafe fn rela(&self) -> Option<&[Rela]> {
        if let Some(slice) = self.pt_dynamic() {
            let mut dt_rela = 0x0;
            let mut dt_relasz = 0x0;
            let mut dt_relaent = 0x0;

            for dt in slice.into_iter() {
                match dt.d_tag {
                    DT_NULL => { break; }
                    DT_RELA => { dt_rela = dt.d_val; }
                    DT_RELASZ => { dt_relasz = dt.d_val; }
                    DT_RELAENT => { dt_relaent = dt.d_val; }
                    _ => { /* ignore */ }
                }
            }

            if dt_rela == 0x0 {
                return None;
            }
            
            return Some(core::slice::from_raw_parts(
                (self.base + dt_rela) as *const Rela,
                (dt_relasz / dt_relaent) as usize
            ));
        }

        None
    }

    pub unsafe fn get_str(&self, idx: usize) -> Option<&str> {
        self.strtab().map(|p| CStr::from_ptr(p.add(idx)).to_str().ok()).flatten()
    }

    pub unsafe fn get_sym(&self, name: &str) -> Option<&Sym> {
        if let Some(slice) = self.dynsym() {
            for sym in slice.into_iter() {
                if Some(name) == self.get_str(sym.st_name as usize) {
                    return Some(sym);
                }
            }
        }

        None
    }

    pub unsafe fn get_sym_addr(&self, name: &str) -> Option<*const u8> {
        self.get_sym(name).map(|sym| (self.base + sym.st_value) as *const u8)
    }

    pub unsafe fn get_sym_name(&self, idx: usize) -> Option<&str> {
        if let Some(slice) = self.dynsym() {
            if let Some(sym) = slice.get(idx) {
                return self.get_str(sym.st_name as usize)
            }
        }

        None
    }
}

