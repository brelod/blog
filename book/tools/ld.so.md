# Implementing the dynamic linker

## Relocating the dynamic loader
So the first problem is that the dynamic loader has to be compiled as static-pie.
If it's compiled as non-pie, than it can not load another non-pie (main) executable because they need to be mapped
into the same memory locations. If it's not compiled as static executable then it needs another dynamic loader to 
load its shared dependencies. But if it's compiled as a static-pie then it has to relocate itself to be able to call
the functions of itself. Now the question is: How can the ld.so relocate itself without referencing any symbols?


## Relocation types:
|Name	            |Value |Field|Calculation
|-------------------|------|-----|-------------------------------|
| R_X86_64_NONE     |0      |None |None                                |
| R_X86_64_64       |1      |qword|`S + A`              |
| R_X86_64_PC32     |2      |dword|`S + A – P`          |
| R_X86_64_GOT32    |3      |dword|`G + A`              |
| R_X86_64_PLT32    |4      |dword|`L + A – P`          |
| R_X86_64_COPY     |5      |None |Value is copied directly from shared object|
| R_X86_64_GLOB_DAT |6      |qword|`S`                                   |
| R_X86_64_JUMP_SLOT|7      |qword|`S`                                   |
| R_X86_64_RELATIVE |8      |qword|`B + A`                          |
| R_X86_64_GOTPCREL |9      |dword|`G + GOT + A – P`|
| R_X86_64_32       |10     |dword|`S + A` |
| R_X86_64_32S      |11     |dword|`S + A` |
| R_X86_64_16       |12     |word |`S + A` |
| R_X86_64_PC16     |13     |word |`S + A – P`          |
| R_X86_64_8        |14     |word8|`S + A`              |
| R_X86_64_PC8      |15     |word8|`S + A – P`          |
| R_X86_64_PC64     |24     |qword|`S + A – P`          |
| R_X86_64_GOTOFF64 |25     |qword|`S + A – GOT`        |
| R_X86_64_GOTPC32  |26     |dword|`GOT + A – P`          |
| R_X86_64_SIZE32   |32     |dword|`Z + A`                          |
| R_X86_64_SIZE64   |33     |qword|`Z + A`                          |

**Shortcuts**:
- A: Addend of Elfxx_Rela entries.
- B: Image base where the shared object was loaded in process virtual address space.
- G: Offset to the GOT relative to the address of the correspondent relocation entry’s symbol.
- GOT: Address of the Global Offset Table
- L: Section offset or address of the procedure linkage table (PLT, .got.plt).
- P: The section offset or address of the storage unit being relocated.  retrieved via r_offset relocation entry’s field.
- S: Relocation entry’s correspondent symbol value.
- Z: Size of Relocations entry’s symbol.

**Suffixes:**
- *_NONE: Neglected entry.
- *_64: qword relocation value.
- *_32: dword relocation value.
- *_16: word relocation value.
- *_8: byte relocation value.
- *_PC: relative to program counter.
- *_GOT: relative to GOT.
- *_PLT: relative to PLT (Procedure Linkage Table).
- *_COPY: value copied directly from shared object at load-time.
- *_GLOB_DAT: global variable.
- *_JMP_SLOT: PLT entry.
- *_RELATIVE: relative to image base of program’s image.
- *_GOTOFF: absolute address within GOT.
- *_GOTPC: program counter relative GOT offset.


## Finding the function name from GOT reference

So we can find in the objdump of our code a relative relocation like this:
```
14b6:ff 15 ac 6a 00 00 call QWORD PTR [rip+0x6aac] #7f68<_GLOBAL_OFFSET_TABLE_+0x88>
```
How can we find out the name of the function? It says that the the address of the function should be read form
the location `_GLOBAL_OFFSET_TABLE_ + 0x88` which has the address `7f68` in our binary file.

This means that we need to have a relative relocation with that exact position which tells us the offset of our
function in the binary file:
```
> readelf -Wr ./target/bin | grep 7f68
0000000000007f68 0000000000000008 R_X86_64_RELATIVE 1790
```
Now we should find the symbol name with `nm` and check for the address `1790` like this:
```
> nm --demangle=rust ./target/bin | grep 1790
0000000000001790 T <linux::types::AT as core::convert::From<linux::types::auxv_t>>::from
```

## Other

Steps:
1. Create binary with PT_INTERP = defined executable
2. Create shared library 
3. Link binary against the share lib
4. Let the ld.so load the shared lib
5. Do symbol resolution in ld.so

Ideas:
- first no-pie ld.so --> doesn't work with no-pie executables
- pie ld.so --> it has to fix itself up first (relocation)

Steps:
1. Setup a dependency tree
2. Load the tree backwards (setup the linkmap)
3. Apply relocations 
4. Start the original executable (cleanup the stack)

Ignore:
- LD_PRELOAD
- LD_LIBRARY_PATH
- dlopen/dlinfo/dlclose
- setup thread local storage
- relocation of the dynamic linker

<div class="warning">
Don't touch the real dynamic linker on your system if you're not 100% sure what you're doing.
Most likely almost all of the programs depends on it so you won't be able to run an ls command without it.
</div>


## Notes:
- Resolve symboles in shared library:
    - code uses `call mysym` and linked with `-no-pie`
    - code uses `call mysym wrt ..plt` and linked with `-pie`
    - code uses `call [rel mysym wrt ..got]` and linked with `-pie` or `-no-pie` (resolution happens without plt through got)
    - https://stackoverflow.com/questions/61278747/is-it-possible-to-access-the-procedure-linkage-table-in-nasm
    - https://stackoverflow.com/questions/52126328/cant-call-c-standard-library-function-on-64-bit-linux-from-assembly-yasm-code

## Starting

Let's create a minimal position independent executable for tests:
```asm
section .text
global _start
_start:
    mov rdi,0x0
    mov rax,0x3c
    syscall
```
Compile and link it like this:
```
> nasm -f elf64 main.s 
> ld --dynamic-linker ld.so -pie ./main.o
```
We can prove the result with readelf and the error seen by running the code
```bash
> readelf -lW ./a.out | grep interpreter
      [Requesting program interpreter: ld.so]

> ./a.out
-bash: ./a.out: No such file or directory
```
The error message is a bit confusing but it means that the dynamic linker has not been found for our executable

So now we only need to provide an ld.so binary which can be found.
Let's create a simple dynamic linker which only prints the pid of the process so we can checkout the memory of it
```rust
#[no_mangle]
fn main() -> u8 { 
    nanosleep(&timespec { tv_sec: 1, tv_nsec: 0 }, &mut timespec::default()).unwrap();
    0
}
```
```
> ./target/a.out & cat /proc/$!/maps
00400000-00401000                  r--p  00000000  fd:00  952545  /target/bin
00401000-00405000                  r-xp  00001000  fd:00  952545  /target/bin
00405000-00407000                  r--p  00005000  fd:00  952545  /target/bin
00407000-00408000                  rw-p  00006000  fd:00  952545  /target/bin
00408000-00409000                  rw-p  00000000  00:00  0       
55abea1e6000-55abea1e7000          r--p  00000000  fd:00  952716  /target/a.out
55abea1e7000-55abea1e8000          r-xp  00001000  fd:00  952716  /target/a.out
55abea1e8000-55abea1e9000          rw-p  00002000  fd:00  952716  /target/a.out
7ffe29afd000-7ffe29b1e000          rw-p  00000000  00:00  0       [stack]
7ffe29b94000-7ffe29b98000          r--p  00000000  00:00  0       [vvar]
7ffe29b98000-7ffe29b9a000          r-xp  00000000  00:00  0       [vdso]
ffffffffff600000-ffffffffff601000  --xp  00000000  00:00  0       [vsyscall]
```

As we can see not just our target executable but our dynamic linker is also mapped into the processes address space. 
But how we can jump to the address of the read executable so it get's run? The answer is in our aux vector: AT_ENTRY.
The AT_ENTRY holds the address of the real executables entry point. So let's parse it and jump to it.
```rust
#[no_mangle]
fn main() -> u8 { 
    for aux in linux::env::auxv() {
        if let AuxvItem::AT_ENTRY(ptr) = aux {
            unsafe {
                let _start: extern "C" fn() -> ! = core::mem::transmute(ptr);
                _start();
            }
        }
    }
    unreachable();
}
```
And if you run the code we can see that the return code is not zero anymore but 255 indicating that jump to the start
function of the a.out.
```
> ./target/a.out
> echo $?
255
```

## Shared lib for GOT
lib.s
```
global rc:data

section .data
rc: db 0xff
```
bin.s
```
global _start
extern rc:data

section .text
_start:
    mov dil,[rel rc wrt ..got]
    mov rax,0x3c
    syscall
```
To compile:
```
> nasm -f elf64 ./target/lib.s -o ./target/lib.o
> nasm -f elf64 ./target/bin.s -o ./target/bin.o
> ld -pic -shared ./target/lib.o -o ./target/lib.so
> ld -pie --dynamic-linker target/bin -L. -l:target/lib.so target/bin.o -o target/a.out
```
Relocations:
```
> readelf -Wr ./target/a.out
Relocation section '.rela.dyn' at offset 0x290 contains 1 entry:
    Offset             Info             Type               Symbol's Value  Symbol's Name + Addend
0000000000002ff8  0000000100000006 R_X86_64_GLOB_DAT      0000000000000000 rc + 0
```

## Shared library for PLT
lib.s
```
global _exit:function

section .text
_exit:
    mov rdi,0xff
    mov rax,0x3c
    syscall
```
bin.s
```
global _start
extern _exit:function

section .text
_start:
    call [rel _exit wrt ..plt]
```
To compile:
```
> nasm -f elf64 ./target/lib.s -o ./target/lib.o
> nasm -f elf64 ./target/bin.s -o ./target/bin.o
> ld -pic -shared ./target/lib.o -o ./target/lib.so
> ld -pie --dynamic-linker target/bin -L. -l:target/lib.so target/bin.o -o target/a.out
```
Relocations:
```
> readelf -Wr ./target/a.out
Relocation section '.rela.plt' at offset 0x290 contains 1 entry:
    Offset             Info             Type               Symbol's Value  Symbol's Name + Addend
0000000000003018  0000000100000007 R_X86_64_JUMP_SLOT     0000000000000000 _exit + 0
```



## Loading
So let's create a shared library an link our binary against it: lib.s
```
global _exit:function

section .text
_exit:
    mov rdi,0xff
    mov rax,0x3c
    syscall
```
and out bin.s looks like this
```
global _start
extern _exit:function

section .text
_start:
    call [rel _exit wrt ..plt]
```
Nasm cheatsheet: https://github.com/hyqneuron/assembler/blob/master/doc/manual/nasm-language.txt
We can compile and link like this:
```
> nasm -f elf64 bin.s
> nasm -f elf64 lib.s
> ld -pic -shared lib.o -o lib.so
> ld -pie --dynamic-linker target/bin -L. -l:target/lib.so target/bin.o -o target/a.out
```
As a result we should get:
```
> readelf -Wd target/a.out
Dynamic section at offset 0x2ec0 contains 15 entries:
  Tag        Type                         Name/Value
 0x0000000000000001 (NEEDED)             Shared library: [target/lib.so]
 0x0000000000000004 (HASH)               0x210
 0x000000006ffffef5 (GNU_HASH)           0x228
 0x0000000000000005 (STRTAB)             0x280
 0x0000000000000006 (SYMTAB)             0x250
 0x000000000000000a (STRSZ)              21 (bytes)
 0x000000000000000b (SYMENT)             24 (bytes)
 0x0000000000000015 (DEBUG)              0x0
 0x0000000000000007 (RELA)               0x298
 0x0000000000000008 (RELASZ)             24 (bytes)
 0x0000000000000009 (RELAENT)            24 (bytes)
 0x0000000000000016 (TEXTREL)            0x0
 0x000000000000001e (FLAGS)              TEXTREL
 0x000000006ffffffb (FLAGS_1)            Flags: PIE
 0x0000000000000000 (NULL)               0x0

> readelf -W --dyn-sym target/a.out
Symbol table '.dynsym' contains 2 entries:
   Num:    Value          Size Type    Bind   Vis      Ndx Name
     0: 0000000000000000     0 NOTYPE  LOCAL  DEFAULT  UND
     1: 0000000000002ec0     0 NOTYPE  GLOBAL DEFAULT    9 _exit
```
So far so good. As next we have to map the shared library as part of the ld.so code

## Linking




## Questions:
- Why is the executable not mapped into the processes memory if it was compiled with no-pie?
    - It seems like in this case only the ld.so get's loaded and it has to load the binary image itself by usign AT_FDEXEC
    - The dynamic linker needs to be compiles as static-pie which can be achived by `-C link-args='-nostartfiles -pie -Wl,--no-dynamic-linker'`
