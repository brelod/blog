# .data

## Asm
```
{{#include examples/elf-data.s}}
```
Compile
```
> nasm -f elf64 elf-data.s
```
Symbols
```
> nm elf-data.o
0000000000000000 D glb
0000000000000001 d loc
```
Sections
```
> readelf -WS ./elf-data.o
Section Headers:
  [Nr] Name      Type     Address          Off    Size   ES Flg Lk Inf Al
  [ 0]           NULL     0000000000000000 000000 000000 00      0   0  0
  [ 1] .data     PROGBITS 0000000000000000 000180 000002 00  WA  0   0  4
  [ 2] .shstrtab STRTAB   0000000000000000 000190 000021 00      0   0  1
  [ 3] .symtab   SYMTAB   0000000000000000 0001c0 000078 18      4   4  8
  [ 4] .strtab   STRTAB   0000000000000000 000240 000014 00      0   0  1
```

## C
```
{{#include examples/elf-data.c}}
```
```
```

## Rust
```
{{#include examples/elf-data.rs}}
```
```
```
