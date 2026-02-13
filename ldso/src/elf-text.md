# .text

## Asm
Source
```
{{#include examples/elf-text.s}}
```
Compilation
```
> nasm -f elf64 elf-text.s
```
Dump
```
> objdump -M intel --disassemble=main elf-text.o
0000000000000000 <main>:
   0:   48 31 c0                xor    rax,rax
   3:   c3                      ret
```
Symbols
```
> readelf -Ws ./elf-text.o
Symbol table '.symtab' contains 4 entries:
   Num:    Value          Size Type    Bind   Vis      Ndx Name
     0: 0000000000000000     0 NOTYPE  LOCAL  DEFAULT  UND
     1: 0000000000000000     0 FILE    LOCAL  DEFAULT  ABS elf-text.s
     2: 0000000000000000     0 SECTION LOCAL  DEFAULT    1 .text
     3: 0000000000000000     0 FUNC    GLOBAL DEFAULT    1 main
```
Sections
```
> readelf -WS ./elf-text.o
Section Headers:
  [Nr] Name              Type            Address          Off    Size   ES Flg Lk Inf Al
  [ 0]                   NULL            0000000000000000 000000 000000 00      0   0  0
  [ 1] .text             PROGBITS        0000000000000000 000180 000004 00  AX  0   0 16
  [ 2] .shstrtab         STRTAB          0000000000000000 000190 000021 00      0   0  1
  [ 3] .symtab           SYMTAB          0000000000000000 0001c0 000060 18      4   4  8
  [ 4] .strtab           STRTAB          0000000000000000 000220 000011 00      0   0  1
```


## C
Source
```
{{#include examples/elf-text.c}}
```
Compilation
```
> gcc elf-text.c -O2 -c
```
Dump
```
> objdump -M intel --disassemble=main elf-text.o
0000000000000000 <main>:
   0:   f3 0f 1e fa             endbr64
   4:   31 c0                   xor    eax,eax
   6:   c3                      ret
```
Symbols
```
> nm elf-text.o
0000000000000000 T main
```

## Rust
Source
```
{{#include examples/elf-text.rs}}
```
Compilation
```
> rustc -O --emit=obj elf-text.rs
```
Dump
```
> objdump -M intel --disassemble=main elf-text.o
0000000000000000 <main>:
   0:   50                      push   rax
   1:   48 89 f1                mov    rcx,rsi
   4:   48 63 d7                movsxd rdx,edi
   7:   48 8d 05 00 00 00 00    lea    rax,[rip+0x0]        # e <main+0xe>
   e:   48 89 04 24             mov    QWORD PTR [rsp],rax
  12:   48 8d 35 00 00 00 00    lea    rsi,[rip+0x0]        # 19 <main+0x19>
  19:   48 89 e7                mov    rdi,rsp
  1c:   45 31 c0                xor    r8d,r8d
  1f:   ff 15 00 00 00 00       call   QWORD PTR [rip+0x0]  # 25 <main+0x25>
  25:   59                      pop    rcx
  26:   c3                      ret
```
Symbols
```
> nm --demangle=rust elf-text.o
0000000000000000 T main
0000000000000000 t std::sys_common::backtrace::__rust_begin_short_backtrace
0000000000000000 T std::rt::lang_start
0000000000000000 t std::rt::lang_start::{{closure}}
                 U std::rt::lang_start_internal
0000000000000000 t core::ops::function::FnOnce::call_once{{vtable.shim}}
0000000000000000 t core::ptr::drop_in_place<std::rt::lang_start<()>::{{closure}}>
0000000000000000 t elf_text::main
```
