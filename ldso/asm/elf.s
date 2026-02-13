
section .bss
struc Ehdr
    .e_ident resb 16
endstruc

section .text


;pub struct Ehdr {
;    pub e_ident: [u8;EI_NIDENT],
;    pub e_type: u16,
;    pub e_machine: u16,
;    pub e_version: u32,
;    pub e_entry: u64,
;    pub e_phoff: u64,
;    pub e_shoff: u64,
;    pub e_flags: u32,
;    pub e_ehsize: u16,
;    pub e_phentsize: u16,
;    pub e_phnum: u16,
;    pub e_shentsize: u16,
;    pub e_shnum: u16,
;    pub e_shstrndx: u16,
;}
