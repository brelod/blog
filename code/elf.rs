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

// e_ident indexes
pub const EI_MAG0:    usize = 0;
pub const EI_MAG1:    usize = 1;
pub const EI_MAG2:    usize = 2;
pub const EI_MAG3:    usize = 3;
pub const EI_CLASS:   usize = 4;
pub const EI_DATA:    usize = 5;
pub const EI_VERSION: usize = 6;
pub const EI_OSABI:   usize = 7;
pub const EI_PAD:     usize = 8;

// Elfs magic string (0x7f ELF)
pub const ELFMAG: [u8;4] = [0x7f, 0x45, 0x4c, 0x46];

// e_ident[EI_CLASS]
pub const ELFCLASSNONE: u8 = 0;
pub const ELFCLASS32:   u8 = 1;
pub const ELFCLASS64:   u8 = 2;
pub const ELFCLASSNUM:  u8 = 3;

// e_ident[EI_DATA]
pub const ELFDATANONE: u8 = 0;
pub const ELFDATA2LSB: u8 = 1;
pub const ELFDATA2MSB: u8 = 2;

// e_ident[EI_VERSION]
pub const EV_NONE:    u8 = 0;
pub const EV_CURRENT: u8 = 1;
pub const EV_NUM:     u8 = 2;

// e_ident[EI_OSABI]
pub const ELFOSABI_NONE:  u8 = 0;
pub const ELFOSABI_LINUX: u8 = 3;

/// Machine type (Ehdr.e_machine)
#[repr(C, align(2))]
#[derive(Debug, Clone)]
pub enum EM {
    EM_NONE   = 0,
    EM_X86_64 = 62,
}

/// File type (Ehdr.e_type)
#[repr(C, align(2))]
#[derive(Debug, Clone)]
pub enum ET {
    ET_NONE   = 0x0000,
    ET_REL    = 0x0001,
    ET_EXEC   = 0x0002,
    ET_DYN    = 0x0003,
    ET_CORE   = 0x0004,
    ET_LOPROC = 0xff00,
    ET_HIPROC = 0xffff,
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

impl Phdr {
    pub fn p_type(&self) -> PT {
        PT::from(self.p_type)
    }

    pub fn flags(&self) -> &'static str {
        match self.p_flags & 0x00000007 {
            0 => "   ",
            1 => "  X",
            2 => " W ",
            3 => " WX",
            4 => "R  ",
            5 => "R X",
            6 => "RW ",
            7 => "RWX",
            _ => unreachable!(),
        }
    }
}

const PT_LOOS:         u32 = 0x60000000;   
const PT_HIOS:         u32 = 0x6fffffff;   
const PT_LOPROC:       u32 = 0x70000000;   
const PT_HIPROC:       u32 = 0x7fffffff;   

const PT_GNU_EH_FRAME: u32 = PT_LOOS + 0x474e550;
const PT_GNU_STACK:    u32 = PT_LOOS + 0x474e551;
const PT_GNU_RELRO:    u32 = PT_LOOS + 0x474e552;
const PT_GNU_PROPERTY: u32 = PT_LOOS + 0x474e553;

/// Program header type (Phdr.p_type)
#[repr(u32)]
#[derive(Debug, Clone)]
pub enum PT {
    PT_NULL         = 0,
    PT_LOAD         = 1,
    PT_DYNAMIC      = 2,
    PT_INTERP       = 3,
    PT_NOTE         = 4,
    PT_SHLIB        = 5,
    PT_PHDR         = 6,
    PT_TLS          = 7,
    PT_GNU_EH_FRAME	= PT_GNU_EH_FRAME,
    PT_GNU_STACK	= PT_GNU_STACK,
    PT_GNU_RELRO	= PT_GNU_RELRO,
    PT_GNU_PROPERTY	= PT_GNU_PROPERTY,
    PT_UNKNOWN      = PT_HIPROC,
}

impl PT {
    fn as_str(&self) -> &'static str {
        match self {
            Self::PT_NULL         => "NULL",
            Self::PT_LOAD         => "LOAD",
            Self::PT_DYNAMIC      => "DYNAMIC",
            Self::PT_INTERP       => "INTERP",
            Self::PT_NOTE         => "NOTE",
            Self::PT_SHLIB        => "SHLIB",
            Self::PT_PHDR         => "PHDR",
            Self::PT_TLS          => "TLS",
            Self::PT_GNU_EH_FRAME => "GNU_EH_FRAME",
            Self::PT_GNU_STACK	  => "GNU_STACK",
            Self::PT_GNU_RELRO	  => "GNU_RELRO",
            Self::PT_GNU_PROPERTY => "GNU_PROPERTY",
            Self::PT_UNKNOWN      => "UNKNOWN",
        }
    }
}

impl From<u32> for PT {
    fn from(n: u32) -> Self {
        match n {
            0                => Self::PT_NULL,
            1                => Self::PT_LOAD,
            2                => Self::PT_DYNAMIC,
            3                => Self::PT_INTERP,
            4                => Self::PT_NOTE,
            5                => Self::PT_SHLIB,
            6                => Self::PT_PHDR,
            7                => Self::PT_TLS,
            PT_GNU_EH_FRAME	 => Self::PT_GNU_EH_FRAME,
            PT_GNU_STACK	 => Self::PT_GNU_STACK,
            PT_GNU_RELRO	 => Self::PT_GNU_RELRO,
            PT_GNU_PROPERTY	 => Self::PT_GNU_PROPERTY,
            _                => Self::PT_UNKNOWN,
        }
    }
}

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

impl Shdr {
    pub fn sh_type(&self) -> SHT {
        SHT::from(self.sh_type)
    }
}

/// Section header type (Shdr.sh_type)
#[repr(u32)] 
#[derive(Debug, Clone)]
pub enum SHT {
    SHT_NULL	        = 0,
    SHT_PROGBITS        = 1,
    SHT_SYMTAB	        = 2,
    SHT_STRTAB	        = 3,
    SHT_RELA	        = 4,
    SHT_HASH	        = 5,
    SHT_DYNAMIC	        = 6,
    SHT_NOTE	        = 7,
    SHT_NOBITS	        = 8,
    SHT_REL		        = 9,
    SHT_SHLIB	        = 10,
    SHT_DYNSYM	        = 11,
    SHT_NUM		        = 12,
    SHT_INIT_ARRAY      = 14,
    SHT_FINI_ARRAY      = 15,
    SHT_PREINIT_ARRAY   = 16,
    SHT_GROUP           = 17,
    SHT_SYMTAB_SHNDX    = 18,
    SHT_GNU_ATTRIBUTES  = 0x6ffffff5,
    SHT_GNU_HASH	    = 0x6ffffff6,
    SHT_GNU_LIBLIST	    = 0x6ffffff7,
    SHT_CHECKSUM	    = 0x6ffffff8,
    SHT_GNU_verdef      = 0x6ffffffd,
    SHT_GNU_verneed     = 0x6ffffffe,
    SHT_GNU_versym      = 0x6fffffff,
    SHT_UNKNOWN         = SHT_HIUSER,
}
pub const SHT_LOPROC: u32 = 0x70000000;
pub const SHT_HIPROC: u32 = 0x7fffffff;
pub const SHT_LOUSER: u32 = 0x80000000;
pub const SHT_HIUSER: u32 = 0xffffffff;

impl From<u32> for SHT {
    fn from(n: u32) -> Self {
        match n {
            0 => Self::SHT_NULL,
            1 => Self::SHT_PROGBITS,
            2 => Self::SHT_SYMTAB,
            3 => Self::SHT_STRTAB,
            4 => Self::SHT_RELA,
            5 => Self::SHT_HASH,
            6 => Self::SHT_DYNAMIC,
            7 => Self::SHT_NOTE,
            8 => Self::SHT_NOBITS,
            9 => Self::SHT_REL,
            10 => Self::SHT_SHLIB,
            11 => Self::SHT_DYNSYM,
            12 => Self::SHT_NUM,
            14 => Self::SHT_INIT_ARRAY,
            15 => Self::SHT_FINI_ARRAY,
            16 => Self::SHT_PREINIT_ARRAY,
            17 => Self::SHT_GROUP,
            18 => Self::SHT_SYMTAB_SHNDX,
            0x6ffffff5 => Self::SHT_GNU_ATTRIBUTES, 
            0x6ffffff6 => Self::SHT_GNU_HASH,
            0x6ffffff7 => Self::SHT_GNU_LIBLIST,
            0x6ffffff8 => Self::SHT_CHECKSUM,
            0x6ffffffd => Self::SHT_GNU_verdef,
            0x6ffffffe => Self::SHT_GNU_verneed,
            0x6fffffff => Self::SHT_GNU_versym,
            _ => Self::SHT_UNKNOWN,
        }
    }
}

impl SHT {
    fn as_str(&self) -> &'static str {
        match self {
            Self::SHT_NULL          => "NULL",
            Self::SHT_PROGBITS      => "PROGBITS",
            Self::SHT_SYMTAB        => "SYMTAB",
            Self::SHT_STRTAB        => "STRTAB",
            Self::SHT_RELA          => "RELA",
            Self::SHT_HASH          => "HASH",
            Self::SHT_DYNAMIC       => "DYNAMIC",
            Self::SHT_NOTE          => "NOTE",
            Self::SHT_NOBITS        => "NOBITS",
            Self::SHT_REL           => "REL",
            Self::SHT_SHLIB         => "SHLIB",
            Self::SHT_DYNSYM        => "DYNSYM",
            Self::SHT_NUM           => "NUM",
            Self::SHT_INIT_ARRAY    => "INIT_ARRAY",
            Self::SHT_FINI_ARRAY    => "FINI_ARRAY",
            Self::SHT_PREINIT_ARRAY => "PREINIT_ARRAY",
            Self::SHT_GROUP         => "GROUP",
            Self::SHT_SYMTAB_SHNDX  => "SYMTAB_SHNDX",
            Self::SHT_GNU_ATTRIBUTES=> "GNU_ATTR", 
            Self::SHT_GNU_HASH      => "GNU_HASH",
            Self::SHT_GNU_LIBLIST   => "GNU_LIBLIST",
            Self::SHT_CHECKSUM      => "CHECKSUM",
            Self::SHT_GNU_verdef    => "VERDEF",
            Self::SHT_GNU_verneed   => "VERNEED",
            Self::SHT_GNU_versym    => "VERSYM",
            Self::SHT_UNKNOWN       => "UNKNOWN",
        }
    }
}

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

impl Sym {
    pub fn st_type(&self) -> STT {
        STT::from(self.st_info)
    }

    pub fn st_bind(&self) -> STB {
        STB::from(self.st_info)
    }

    pub fn st_visibility(&self) -> STV {
        STV::from(self.st_other)
    }
}

/// Symbol type (Sym.st_info)
#[repr(u8)]
#[derive(Debug, Clone)]
pub enum STT {
    STT_NOTYPE  = 0,
    STT_OBJECT  = 1,
    STT_FUNC    = 2,
    STT_SECTION = 3,
    STT_FILE    = 4,
    STT_COMMON  = 5,
    STT_TLS     = 6,
    STT_UNKNOWN = 255,
}

impl From<u8> for STT {
    fn from(n: u8) -> Self {
        match n & 0xf {
            0 => Self::STT_NOTYPE,
            1 => Self::STT_OBJECT,
            2 => Self::STT_FUNC,
            3 => Self::STT_SECTION,
            4 => Self::STT_FILE,
            5 => Self::STT_COMMON,
            6 => Self::STT_TLS,
            _ => Self::STT_UNKNOWN,
        }
    }
}

impl STT {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::STT_NOTYPE  => "NOTYPE",
            Self::STT_OBJECT  => "OBJECT",
            Self::STT_FUNC    => "FUNC",
            Self::STT_SECTION => "SECTION",
            Self::STT_FILE    => "FILE",
            Self::STT_COMMON  => "COMMON",
            Self::STT_TLS     => "TLS",
            Self::STT_UNKNOWN => "UNKNOWN",
        }
    }
}

/// Symbol binding (Sym.st_info)
#[repr(u8)]
#[derive(Debug, Clone)]
pub enum STB {
    STB_LOCAL  = 0,
    STB_GLOBAL = 1,
    STB_WEAK   = 2,
    STB_UNKNOWN= 255,
}

impl From<u8> for STB {
    fn from(n: u8) -> Self {
        match n >> 4 {
            0 => Self::STB_LOCAL,
            1 => Self::STB_GLOBAL,
            2 => Self::STB_WEAK,
            _ => Self::STB_UNKNOWN,
        }
    }
}

impl STB {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::STB_LOCAL  => "LOCAL",
            Self::STB_GLOBAL => "GLOBAL",
            Self::STB_WEAK   => "WEAK",
            Self::STB_UNKNOWN=> "UNKNOWN",
        }
    }
}


/// Symbol visibility (Sym.st_other)
#[repr(u8)]
#[derive(Debug, Clone)]
pub enum STV {
    STV_DEFAULT = 0,
    STV_INTERNAL= 1,
    STV_HIDDEN  = 2,
    STV_PROTECTED = 3,
    STV_UNKNOWN = 255,
}

impl From<u8> for STV {
    fn from(n: u8) -> Self {
        match n & 0x03 {
            0 => Self::STV_DEFAULT,
            1 => Self::STV_INTERNAL,
            2 => Self::STV_HIDDEN,
            3 => Self::STV_PROTECTED,
            _ => Self::STV_UNKNOWN,
        }
    }
}

impl STV {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::STV_DEFAULT   => "DEFAULT",
            Self::STV_INTERNAL  => "INTERNAL",
            Self::STV_HIDDEN    => "HIDDEN",
            Self::STV_PROTECTED => "PROTECTED",
            Self::STV_UNKNOWN   => "UNKNOWN",
        }
    }
}

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

impl Dyn {
    pub fn d_tag(&self) -> DT {
        DT::from(self.d_tag)
    }
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

pub const DT_NULL             : i64 = 0;
pub const DT_NEEDED           : i64 = 1;
pub const DT_PLTRELSZ         : i64 = 2;
pub const DT_PLTGOT           : i64 = 3;
pub const DT_HASH             : i64 = 4;
pub const DT_STRTAB           : i64 = 5;
pub const DT_SYMTAB           : i64 = 6;
pub const DT_RELA             : i64 = 7;
pub const DT_RELASZ           : i64 = 8;
pub const DT_RELAENT          : i64 = 9;
pub const DT_STRSZ            : i64 = 10;
pub const DT_SYMENT           : i64 = 11;
pub const DT_INIT             : i64 = 12;
pub const DT_FINI             : i64 = 13;
pub const DT_SONAME           : i64 = 14;
pub const DT_RPATH            : i64 = 15;
pub const DT_SYMBOLIC         : i64 = 16;
pub const DT_REL              : i64 = 17;
pub const DT_RELSZ            : i64 = 18;
pub const DT_RELENT           : i64 = 19;
pub const DT_PLTREL           : i64 = 20;
pub const DT_DEBUG            : i64 = 21;
pub const DT_TEXTREL          : i64 = 22;
pub const DT_JMPREL           : i64 = 23;
pub const DT_BIND_NOW	      : i64   = 24;
pub const DT_INIT_ARRAY       : i64 = 25;
pub const DT_FINI_ARRAY       : i64 = 26;
pub const DT_INIT_ARRAYSZ     : i64 = 27;
pub const DT_FINI_ARRAYSZ     : i64 = 28;
pub const DT_RUNPATH          : i64 = 29;
pub const DT_FLAGS            : i64 = 30;
pub const DT_ENCODING         : i64 = 32;
pub const DT_PREINIT_ARRAYSZ  : i64 = 33;
pub const DT_NUM              : i64 = 34;
pub const DT_GNU_HASH         : i64 = 0x6ffffef5;
pub const DT_VERSYM           : i64 = 0x6ffffff0;
pub const DT_RELACOUNT        : i64 = 0x6ffffff9;
pub const DT_RELCOUNT         : i64 = 0x6ffffffa;
pub const DT_FLAGS_1          : i64 = 0x6ffffffb;
pub const DT_VERDEF           : i64 = 0x6ffffffc;
pub const DT_VERDEFNUM        : i64 = 0x6ffffffd;
pub const DT_VERNEED          : i64 = 0x6ffffffe;
pub const DT_VERNEEDNUM       : i64 = 0x6fffffff;
pub const DT_UNKNOWN          : i64 = DT_HIPROC;

/// Dynamic tag (Dyn.d_tag)
#[repr(i64)]
#[derive(Debug, Clone)]
pub enum DT {
    DT_NULL             = 0,
    DT_NEEDED           = 1,
    DT_PLTRELSZ         = 2,
    DT_PLTGOT           = 3,
    DT_HASH             = 4,
    DT_STRTAB           = 5,
    DT_SYMTAB           = 6,
    DT_RELA             = 7,
    DT_RELASZ           = 8,
    DT_RELAENT          = 9,
    DT_STRSZ            = 10,
    DT_SYMENT           = 11,
    DT_INIT             = 12,
    DT_FINI             = 13,
    DT_SONAME           = 14,
    DT_RPATH            = 15,
    DT_SYMBOLIC         = 16,
    DT_REL              = 17,
    DT_RELSZ            = 18,
    DT_RELENT           = 19,
    DT_PLTREL           = 20,
    DT_DEBUG            = 21,
    DT_TEXTREL          = 22,
    DT_JMPREL           = 23,
    DT_BIND_NOW	        = 24,
    DT_INIT_ARRAY       = 25,
    DT_FINI_ARRAY       = 26,
    DT_INIT_ARRAYSZ     = 27,
    DT_FINI_ARRAYSZ     = 28,
    DT_RUNPATH          = 29,
    DT_FLAGS            = 30,
    DT_ENCODING         = 32,
    DT_PREINIT_ARRAYSZ  = 33,
    DT_NUM              = 34,
    DT_GNU_HASH         = 0x6ffffef5,
    DT_VERSYM           = 0x6ffffff0,
    DT_RELACOUNT        = 0x6ffffff9,
    DT_RELCOUNT         = 0x6ffffffa,
    DT_FLAGS_1          = 0x6ffffffb,
    DT_VERDEF           = 0x6ffffffc,
    DT_VERDEFNUM        = 0x6ffffffd,
    DT_VERNEED          = 0x6ffffffe,
    DT_VERNEEDNUM       = 0x6fffffff,
    DT_UNKNOWN          = DT_HIPROC,
}

impl From<i64> for DT {
    fn from(n: i64) -> Self {
        match n {
            0          => Self::DT_NULL,
            1          => Self::DT_NEEDED,
            2          => Self::DT_PLTRELSZ,
            3          => Self::DT_PLTGOT,
            4          => Self::DT_HASH,
            5          => Self::DT_STRTAB,
            6          => Self::DT_SYMTAB,
            7          => Self::DT_RELA,
            8          => Self::DT_RELASZ,
            9          => Self::DT_RELAENT,
            10         => Self::DT_STRSZ,
            11         => Self::DT_SYMENT,
            12         => Self::DT_INIT,
            13         => Self::DT_FINI,
            14         => Self::DT_SONAME,
            15         => Self::DT_RPATH,
            16         => Self::DT_SYMBOLIC,
            17         => Self::DT_REL,
            18         => Self::DT_RELSZ,
            19         => Self::DT_RELENT,
            20         => Self::DT_PLTREL,
            21         => Self::DT_DEBUG,
            22         => Self::DT_TEXTREL,
            23         => Self::DT_JMPREL,
            24         => Self::DT_BIND_NOW,
            25         => Self::DT_INIT_ARRAY,
            26         => Self::DT_FINI_ARRAY,
            27         => Self::DT_INIT_ARRAYSZ,
            28         => Self::DT_FINI_ARRAYSZ,
            29         => Self::DT_RUNPATH,
            30         => Self::DT_FLAGS,
            32         => Self::DT_ENCODING,
            33         => Self::DT_PREINIT_ARRAYSZ,
            34         => Self::DT_NUM,
            0x6ffffef5 => Self::DT_GNU_HASH,
            0x6ffffff0 => Self::DT_VERSYM,
            0x6ffffff9 => Self::DT_RELACOUNT,
            0x6ffffffa => Self::DT_RELCOUNT,
            0x6ffffffb => Self::DT_FLAGS_1,
            0x6ffffffc => Self::DT_VERDEF,
            0x6ffffffd => Self::DT_VERDEFNUM,
            0x6ffffffe => Self::DT_VERNEED,
            0x6fffffff => Self::DT_VERNEEDNUM,
            _          => Self::DT_UNKNOWN,
        }
    }
}

impl DT {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::DT_NULL           => "NULL",
            Self::DT_NEEDED         => "NEEDED",
            Self::DT_PLTRELSZ       => "PLTRELSZ",
            Self::DT_PLTGOT         => "PLTGOT",
            Self::DT_HASH           => "HASH",
            Self::DT_STRTAB         => "STRTAB",
            Self::DT_SYMTAB         => "SYMTAB",
            Self::DT_RELA           => "RELA",
            Self::DT_RELASZ         => "RELASZ",
            Self::DT_RELAENT        => "RELAENT",
            Self::DT_STRSZ          => "STRSZ",
            Self::DT_SYMENT         => "SYMENT",
            Self::DT_INIT           => "INIT",
            Self::DT_FINI           => "FINI",
            Self::DT_SONAME         => "SONAME",
            Self::DT_RPATH          => "RPATH",
            Self::DT_SYMBOLIC       => "SYMBOLIC",
            Self::DT_REL            => "REL",
            Self::DT_RELSZ          => "RELSZ",
            Self::DT_RELENT         => "RELENT",
            Self::DT_PLTREL         => "PLTREL",
            Self::DT_DEBUG          => "DEBUG",
            Self::DT_TEXTREL        => "TEXTREL",
            Self::DT_JMPREL         => "JMPREL",
            Self::DT_BIND_NOW       => "BIND_NOW",
            Self::DT_INIT_ARRAY     => "INIT_ARRAY",
            Self::DT_FINI_ARRAY     => "FINI_ARRAY",
            Self::DT_INIT_ARRAYSZ   => "INIT_ARRAYSZ",
            Self::DT_FINI_ARRAYSZ   => "FINI_ARRAYSZ",
            Self::DT_RUNPATH        => "RUNPATH",
            Self::DT_FLAGS          => "FLAGS",
            Self::DT_ENCODING       => "ENCODING",
            Self::DT_PREINIT_ARRAYSZ=> "PREINIT_ARRAYSZ",
            Self::DT_NUM            => "NUM",
            Self::DT_GNU_HASH       => "GNU_HASH",
            Self::DT_VERSYM         => "VERSYM",
            Self::DT_RELACOUNT      => "RELACOUNT",
            Self::DT_RELCOUNT       => "RELCOUNT",
            Self::DT_FLAGS_1        => "FLAGS_1",
            Self::DT_VERDEF         => "VERDEF",
            Self::DT_VERDEFNUM      => "VERDEFNUM",
            Self::DT_VERNEED        => "VERNEED",
            Self::DT_VERNEEDNUM     => "VERNEEDNUM",
            Self::DT_UNKNOWN        => "UNKNOWN",
        }
    }
}


// ==============================================================================
// Relocation table
// ==============================================================================
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Rel {
    pub r_offset: u64,
    pub r_info: u64,
}

impl Rel {
    pub fn r_type(&self) -> R {
        R::from((self.r_info & 0xffffffff) as u32)
    }

    pub fn r_sym(&self) -> u32 {
        (self.r_info >> 32) as u32
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Rela {
    pub r_offset: u64,
    pub r_info: u64,
    pub r_addend: i64,
}

impl Rela {
    pub fn r_type(&self) -> R {
        R::from((self.r_info & 0xffffffff) as u32)
    }

    pub fn r_sym(&self) -> u32 {
        (self.r_info >> 32) as u32
    }
}

#[repr(u32)]
#[derive(Debug, Clone)]
pub enum R {
    R_X86_64_NONE           = 0,    /* No reloc */
    R_X86_64_64             = 1,    /* Direct 64 bit  */
    R_X86_64_PC32           = 2,    /* PC relative 32 bit signed */
    R_X86_64_GOT32          = 3,    /* 32 bit GOT entry */
    R_X86_64_PLT32          = 4,    /* 32 bit PLT address */
    R_X86_64_COPY           = 5,    /* Copy symbol at runtime */
    R_X86_64_GLOB_DAT       = 6,    /* Create GOT entry */
    R_X86_64_JUMP_SLOT      = 7,    /* Create PLT entry */
    R_X86_64_RELATIVE       = 8,    /* Adjust by program base */
    R_X86_64_GOTPCREL       = 9,    /* 32 bit signed PC relative offset to GOT */
    R_X86_64_32             = 10,   /* Direct 32 bit zero extended */
    R_X86_64_32S            = 11,   /* Direct 32 bit sign extended */
    R_X86_64_16             = 12,   /* Direct 16 bit zero extended */
    R_X86_64_PC16           = 13,   /* 16 bit sign extended pc relative */
    R_X86_64_8              = 14,   /* Direct 8 bit sign extended  */
    R_X86_64_PC8            = 15,   /* 8 bit sign extended pc relative */
    R_X86_64_DTPMOD64       = 16,   /* ID of module containing symbol */
    R_X86_64_DTPOFF64       = 17,   /* Offset in module's TLS block */
    R_X86_64_TPOFF64        = 18,   /* Offset in initial TLS block */
    R_X86_64_TLSGD          = 19,   /* 32 bit signed PC relative offset to two GOT entries for GD symbol */
    R_X86_64_TLSLD          = 20,   /* 32 bit signed PC relative offset to two GOT entries for LD symbol */
    R_X86_64_DTPOFF32       = 21,   /* Offset in TLS block */
    R_X86_64_GOTTPOFF       = 22,   /* 32 bit signed PC relative offset to GOT entry for IE symbol */
    R_X86_64_TPOFF32        = 23,   /* Offset in initial TLS block */
    R_X86_64_PC64           = 24,   /* PC relative 64 bit */
    R_X86_64_GOTOFF64       = 25,   /* 64 bit offset to GOT */
    R_X86_64_GOTPC32        = 26,   /* 32 bit signed pc relative offset to GOT */
    R_X86_64_GOT64          = 27,   /* 64-bit GOT entry offset */
    R_X86_64_GOTPCREL64     = 28,   /* 64-bit PC relative offset to GOT entry */
    R_X86_64_GOTPC64        = 29,   /* 64-bit PC relative offset to GOT */
    R_X86_64_GOTPLT64       = 30,   /* like GOT64, says PLT entry needed */
    R_X86_64_PLTOFF64       = 31,   /* 64-bit GOT relative offset to PLT entry */
    R_X86_64_SIZE32         = 32,   /* Size of symbol plus 32-bit addend */
    R_X86_64_SIZE64         = 33,   /* Size of symbol plus 64-bit addend */
    R_X86_64_GOTPC32_TLSDESC= 34,   /* GOT offset for TLS descriptor.  */
    R_X86_64_TLSDESC_CALL   = 35,   /* Marker for call through TLS descriptor.  */
    R_X86_64_TLSDESC        = 36,   /* TLS descriptor.  */
    R_X86_64_IRELATIVE      = 37,   /* Adjust indirectly by program base */
    R_X86_64_RELATIVE64     = 38,   /* 64-bit adjust by program base */ /* 39 Reserved was R_X86_64_PC32_BND */ /* 40 Reserved was R_X86_64_PLT32_BND */
    R_X86_64_GOTPCRELX      = 41,   /* Load from 32 bit signed pc relative offset to GOT entry without REX prefix, relaxable.  */
    R_X86_64_REX_GOTPCRELX  = 42,   /* Load from 32 bit signed pc relative offset to GOT entry with REX prefix, relaxable.  */
    R_X86_64_UNKNOWN        = u32::MAX,
}


impl From<u32> for R {
    fn from(n: u32) -> Self {
        match n  {
            0  => Self::R_X86_64_NONE,
            1  => Self::R_X86_64_64,
            2  => Self::R_X86_64_PC32,
            3  => Self::R_X86_64_GOT32,
            4  => Self::R_X86_64_PLT32,
            5  => Self::R_X86_64_COPY,
            6  => Self::R_X86_64_GLOB_DAT,
            7  => Self::R_X86_64_JUMP_SLOT,
            8  => Self::R_X86_64_RELATIVE,
            9  => Self::R_X86_64_GOTPCREL,
            10 => Self::R_X86_64_32,
            11 => Self::R_X86_64_32S,
            12 => Self::R_X86_64_16,
            13 => Self::R_X86_64_PC16,
            14 => Self::R_X86_64_8,
            15 => Self::R_X86_64_PC8,
            16 => Self::R_X86_64_DTPMOD64,
            17 => Self::R_X86_64_DTPOFF64,
            18 => Self::R_X86_64_TPOFF64,
            19 => Self::R_X86_64_TLSGD,
            20 => Self::R_X86_64_TLSLD,
            21 => Self::R_X86_64_DTPOFF32,
            22 => Self::R_X86_64_GOTTPOFF,
            23 => Self::R_X86_64_TPOFF32,
            24 => Self::R_X86_64_PC64,
            25 => Self::R_X86_64_GOTOFF64,
            26 => Self::R_X86_64_GOTPC32,
            27 => Self::R_X86_64_GOT64,
            28 => Self::R_X86_64_GOTPCREL64,
            29 => Self::R_X86_64_GOTPC64,
            30 => Self::R_X86_64_GOTPLT64,
            31 => Self::R_X86_64_PLTOFF64,
            32 => Self::R_X86_64_SIZE32,
            33 => Self::R_X86_64_SIZE64,
            34 => Self::R_X86_64_GOTPC32_TLSDESC,
            35 => Self::R_X86_64_TLSDESC_CALL,
            36 => Self::R_X86_64_TLSDESC,
            37 => Self::R_X86_64_IRELATIVE,
            38 => Self::R_X86_64_RELATIVE64,
            41 => Self::R_X86_64_GOTPCRELX,
            42 => Self::R_X86_64_REX_GOTPCRELX,
            _  => Self::R_X86_64_UNKNOWN,
        }
    }
}

impl R {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::R_X86_64_NONE            => "X86_64_NONE",
            Self::R_X86_64_64              => "X86_64_64",
            Self::R_X86_64_PC32            => "X86_64_PC32",
            Self::R_X86_64_GOT32           => "X86_64_GOT32",
            Self::R_X86_64_PLT32           => "X86_64_PLT32",
            Self::R_X86_64_COPY            => "X86_64_COPY",
            Self::R_X86_64_GLOB_DAT        => "X86_64_GLOB_DAT",
            Self::R_X86_64_JUMP_SLOT       => "X86_64_JUMP_SLOT",
            Self::R_X86_64_RELATIVE        => "X86_64_RELATIVE",
            Self::R_X86_64_GOTPCREL        => "X86_64_GOTPCREL",
            Self::R_X86_64_32              => "X86_64_32",
            Self::R_X86_64_32S             => "X86_64_32S",
            Self::R_X86_64_16              => "X86_64_16",
            Self::R_X86_64_PC16            => "X86_64_PC16",
            Self::R_X86_64_8               => "X86_64_8",
            Self::R_X86_64_PC8             => "X86_64_PC8",
            Self::R_X86_64_DTPMOD64        => "X86_64_DTPMOD64",
            Self::R_X86_64_DTPOFF64        => "X86_64_DTPOFF64",
            Self::R_X86_64_TPOFF64         => "X86_64_TPOFF64",
            Self::R_X86_64_TLSGD           => "X86_64_TLSGD",
            Self::R_X86_64_TLSLD           => "X86_64_TLSLD",
            Self::R_X86_64_DTPOFF32        => "X86_64_DTPOFF32",
            Self::R_X86_64_GOTTPOFF        => "X86_64_GOTTPOFF",
            Self::R_X86_64_TPOFF32         => "X86_64_TPOFF32",
            Self::R_X86_64_PC64            => "X86_64_PC64",
            Self::R_X86_64_GOTOFF64        => "X86_64_GOTOFF64",
            Self::R_X86_64_GOTPC32         => "X86_64_GOTPC32",
            Self::R_X86_64_GOT64           => "X86_64_GOT64",
            Self::R_X86_64_GOTPCREL64      => "X86_64_GOTPCREL64",
            Self::R_X86_64_GOTPC64         => "X86_64_GOTPC64",
            Self::R_X86_64_GOTPLT64        => "X86_64_GOTPLT64",
            Self::R_X86_64_PLTOFF64        => "X86_64_PLTOFF64",
            Self::R_X86_64_SIZE32          => "X86_64_SIZE32",
            Self::R_X86_64_SIZE64          => "X86_64_SIZE64",
            Self::R_X86_64_GOTPC32_TLSDESC => "X86_64_GOTPC32_TLSDESC",
            Self::R_X86_64_TLSDESC_CALL    => "X86_64_TLSDESC_CALL",
            Self::R_X86_64_TLSDESC         => "X86_64_TLSDESC",
            Self::R_X86_64_IRELATIVE       => "X86_64_IRELATIVE",
            Self::R_X86_64_RELATIVE64      => "X86_64_RELATIVE64",
            Self::R_X86_64_GOTPCRELX       => "X86_64_GOTPCRELX",
            Self::R_X86_64_REX_GOTPCRELX   => "X86_64_REX_GOTPCRELX",
            Self::R_X86_64_UNKNOWN         => "X86_64_UNKNOWN",
        }
    }
}
// ==============================================================================
// Notes table
// ==============================================================================
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Note {
    n_namesz: u32,
    n_descsz: u32,
    n_type: u32,
}

#[repr(u32)]
#[derive(Debug, Clone)]
pub enum NT {
    NT_PRSTATUS	           = 0x00000001,
    NT_PRFPREG	           = 0x00000002,
    NT_PRPSINFO	           = 0x00000003,
    NT_TASKSTRUCT	       = 0x00000004,
    NT_GNU_PROPERTY_TYPE_0 = 0x00000005,
    NT_AUXV		           = 0000000006,
    NT_SIGINFO             = 0x53494749,
    NT_FILE                = 0x46494c45,
    NT_PRXFPREG            = 0x46e62b7f, /* copied from gdb5.1/include/elf/common.h */
    NT_386_TLS	           = 0x00000200, /* i386 TLS slots (struct user_desc) */
    NT_386_IOPERM	       = 0x00000201, /* x86 io permission bitmap (1=deny) */
    NT_X86_XSTATE	       = 0x00000202, /* x86 extended state using xsave */
    NT_X86_SHSTK	       = 0x00000204, /* x86 SHSTK state */
    NT_VMCOREDD	           = 0x00000700, /* Vmcore Device Dump Note */
    NT_MIPS_DSP	           = 0x00000800, /* MIPS DSP ASE registers */
    NT_MIPS_FP_MODE	       = 0x00000801, /* MIPS floating-point mode */
    NT_MIPS_MSA	           = 0x00000802, /* MIPS SIMD registers */
    NT_RISCV_CSR	       = 0x00000900, /* RISC-V Control and Status Registers */
    NT_RISCV_VECTOR	       = 0x00000901, /* RISC-V vector registers */
    NT_LOONGARCH_CPUCFG	   = 0x00000a00, /* LoongArch CPU config registers */
    NT_LOONGARCH_CSR	   = 0x00000a01, /* LoongArch control and status registers */
    NT_LOONGARCH_LSX	   = 0x00000a02, /* LoongArch Loongson SIMD Extension registers */
    NT_LOONGARCH_LASX	   = 0x00000a03, /* LoongArch Loongson Advanced SIMD Extension registers */
    NT_LOONGARCH_LBT	   = 0x00000a04, /* LoongArch Loongson Binary Translation registers */
    NT_LOONGARCH_HW_BREAK  = 0x00000a05, /* LoongArch hardware breakpoint registers */
    NT_LOONGARCH_HW_WATCH  = 0x00000a06, /* LoongArch hardware watchpoint registers */
    NT_UNKNOWN             = 0xffffffff, 
}


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

    pub fn rel(&self) -> &[Rel] {
        // TODO: Fix this bug: This returns only with the first relocation table but there might be more
        for h in self.shdrs.into_iter() {
            if h.sh_type == SHT::SHT_REL as u32 {
                return unsafe {
                    let p = (self.ehdr as *const _ as *const i8)
                        .add(h.sh_offset as usize) as *const Rel;
                    core::slice::from_raw_parts(p, (h.sh_size / h.sh_entsize) as usize)
                };
            }
        }

        &[]
    }

    pub fn rela(&self) -> &[Rela] {
        // TODO: Fix this bug: This returns only with the first relocation table but there might be more
        for h in self.shdrs.into_iter() {
            if h.sh_type == SHT::SHT_RELA as u32 {
                return unsafe {
                    let p = (self.ehdr as *const _ as *const i8)
                        .add(h.sh_offset as usize) as *const Rela;
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

    pub fn dump_rel(&self) {
        let slice = self.rel();
        crate::println!("Dynamic reloaction: {:?}", slice.len());

        if slice.len() == 0 {
            return;
        }

        crate::println!("  {:<18} {:<18} {:<22} {:<10} {}", "Offset", "Info", "Type", "Symbol", "Name");
        for e in slice.into_iter() {
            crate::println!("  0x{:0>16x} {:0>16x} {:<22} 0x{:0>8} {}", 
                e.r_offset,
                e.r_info,
                e.r_type().as_str(),
                e.r_sym(),
                "hardcoded-symbol-name"
            );
        }
    }

    pub fn dump_rela(&self) {
        let slice = self.rela();
        crate::println!("Dynamic reloaction addend: {:?}", slice.len());

        if slice.len() == 0 {
            return;
        }

        crate::println!("  {:<18} {:<18} {:<6} {:<22} {:<10} {}", "Offset", "Info", "Addend", "Type", "Symbol", "Name");
        for e in slice.into_iter() {
            let sym = &self.dynsym()[e.r_sym() as usize];
            crate::println!("  0x{:0>16x} 0x{:0>16x} {:<6} {:<22} 0x{:0>8} {}", 
                e.r_offset,
                e.r_info,
                e.r_addend,
                e.r_type().as_str(),
                e.r_sym(),
                unsafe { self.strtab(sym.st_name) }
            );
        }
    }
}

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
            if h.p_type == PT::PT_DYNAMIC as u32 {
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
            if h.sh_type == SHT::SHT_DYNSYM as u32 {
                return Some(core::slice::from_raw_parts(
                    (self.base + h.sh_offset) as *const Sym,
                    (h.sh_size / h.sh_entsize) as usize,
                ));
            }
        }

        None
    }

    pub unsafe fn rela(&self) -> Option<&[Rela]> {
        if let Some(slice) = self.pt_dynamic() {
            let mut dt_rela = 0x0;
            let mut dt_relasz = 0x0;
            let mut dt_relaent = 0x0;

            for dt in slice.into_iter() {
                match dt.d_tag {
                    DT_RELA => { dt_rela = dt.d_val; }
                    DT_RELASZ => { dt_relasz = dt.d_val; }
                    DT_RELAENT => { dt_relaent = dt.d_val; }
                    _ => { /* ignore */ }
                }
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

