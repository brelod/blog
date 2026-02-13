# The dynamic linker

In these series we're going to implement a basic version of a dynamic linker to load and relocate symbols at runtime.
Let's make a step back and see what's dynamic linking all about. As always, we're going to use the simplest example programs
and for that we have to write some assembly. Let's get some definitions done to avoid confusion:

## Object file:
An object file is a compilation unit in which all the necessary information is collected.
Once the program files are compiled and organized into object these files can be linked together to form a library 
or an executable file. This job is done by the program linker.

Let's create a simplistic object file which only holds a global variable. We could use this variable to specify the exit
code fo the program so let's call the file `rc.s` and the variable `RC`
```
global RC:data
section .data
RC: db 1
```
You can compile and print the most important information about the object file like this: (I removed some unimportant lines)
```
> nasm -f elf64 rc.s
> readelf -Wa rc.o
ELF Header:
  Magic:   7f 45 4c 46 02 01 01 00 00 00 00 00 00 00 00 00
  Class:                             ELF64
  Data:                              2's complement, little endian
  Version:                           1 (current)
  OS/ABI:                            UNIX - System V
  ABI Version:                       0
  Type:                              REL (Relocatable file)
  Machine:                           Advanced Micro Devices X86-64
  Version:                           0x1
  Entry point address:               0x0
  Start of program headers:          0 (bytes into file)
  Start of section headers:          64 (bytes into file)
  Flags:                             0x0
  Size of this header:               64 (bytes)
  Size of program headers:           0 (bytes)
  Number of program headers:         0
  Size of section headers:           64 (bytes)
  Number of section headers:         5
  Section header string table index: 2

Section Headers:
  [Nr] Name      Type     Address          Off    Size   ES Flg Lk Inf Al
  [ 0]           NULL     0000000000000000 000000 000000 00      0   0  0
  [ 1] .data     PROGBITS 0000000000000000 000180 000001 00  WA  0   0  4
  [ 2] .shstrtab STRTAB   0000000000000000 000190 000021 00      0   0  1
  [ 3] .symtab   SYMTAB   0000000000000000 0001c0 000060 18      4   3  8
  [ 4] .strtab   STRTAB   0000000000000000 000220 00000b 00      0   0  1

Symbol table '.symtab' contains 4 entries:
   Num:    Value          Size Type    Bind   Vis      Ndx Name
     0: 0000000000000000     0 NOTYPE  LOCAL  DEFAULT  UND
     1: 0000000000000000     0 FILE    LOCAL  DEFAULT  ABS ./rc.s
     2: 0000000000000000     0 SECTION LOCAL  DEFAULT    1 .data
     3: 0000000000000000     0 OBJECT  GLOBAL DEFAULT    1 RC
```
This tells us the followings:
- The type of the file is `REL (Relocatable file)`
- It has a `.data` section (`section .data` in asm)
- It has a global symbol called `RC` (last line) and it's located in the fist section (Ndx=1) which is the `.data` section

As you can see the value of all the symbols are zero. The value should be a memory addres which the symbol points to, so how
can it be zero? It will be updated by the linker once this object file is merged into an executable or a shared library.

## Static linking:
The simplest way to create an elf binary is merging all of its parts into a single file. This allows it to be fully independent
from any other userspace code. As a result you can put it into a docker container / chroot environment and it will just run.

Let's reimplement the `/bin/false` command in such a way: The only thin the `false` command does is exiting with 1 as return code.
To make it a bit more interesting let's use the `rc.o` file as a static library which we can include into our binary and use the
value of `RC` defined there as the exit code of our binary. The source of our `false.s` looks lie this:
```
global _start
extern RC:data

section .text
_start:
    mov rdi,[RC]
    mov rax,0x3c
    syscall
```
With `extern RC:data` we tell the assembler that the `RC` with type `data` exists somewhere in another object file which we will
link against. With `mov rdi,[RC]` we say the compiler to go to the address marked by `RC` and read the value of the memory there
and move it intot the rdi register. This register is used as the return value of the exit system call. 
We can compile, link and run like this:
```
> nasm -f elf64 ./false.s
> ld -static ./false.o ./rc.o -o ./false
> ./false; echo $?
1
```
Let's have a closer look with `readelf`
```
> readelf -Wa ./false
ELF Header:
  Magic:   7f 45 4c 46 02 01 01 00 00 00 00 00 00 00 00 00
  Class:                             ELF64
  Data:                              2's complement, little endian
  Version:                           1 (current)
  OS/ABI:                            UNIX - System V
  ABI Version:                       0
  Type:                              EXEC (Executable file)
  Machine:                           Advanced Micro Devices X86-64
  Version:                           0x1
  Entry point address:               0x401000
  Start of program headers:          64 (bytes into file)
  Start of section headers:          8480 (bytes into file)
  Flags:                             0x0
  Size of this header:               64 (bytes)
  Size of program headers:           56 (bytes)
  Number of program headers:         3
  Size of section headers:           64 (bytes)
  Number of section headers:         6
  Section header string table index: 5

Section Headers:
  [Nr] Name      Type     Address          Off    Size   ES Flg Lk Inf Al
  [ 0]           NULL     0000000000000000 000000 000000 00      0   0  0
  [ 1] .text     PROGBITS 0000000000401000 001000 00000f 00  AX  0   0 16
  [ 2] .data     PROGBITS 0000000000402000 002000 000001 00  WA  0   0  4
  [ 3] .symtab   SYMTAB   0000000000000000 002008 0000c0 18      4   3  8
  [ 4] .strtab   STRTAB   0000000000000000 0020c8 00002d 00      0   0  1
  [ 5] .shstrtab STRTAB   0000000000000000 0020f5 000027 00      0   0  1

Program Headers:
  Type Offset   VirtAddr           PhysAddr           FileSiz  MemSiz   Flg Align
  LOAD 0x000000 0x0000000000400000 0x0000000000400000 0x0000e8 0x0000e8 R   0x1000
  LOAD 0x001000 0x0000000000401000 0x0000000000401000 0x00000f 0x00000f R E 0x1000
  LOAD 0x002000 0x0000000000402000 0x0000000000402000 0x000001 0x000001 RW  0x1000

 Section to Segment mapping:
  Segment Sections...
   00
   01     .text
   02     .data

Symbol table '.symtab' contains 8 entries:
   Num:    Value          Size Type    Bind   Vis      Ndx Name
     0: 0000000000000000     0 NOTYPE  LOCAL  DEFAULT  UND
     1: 0000000000000000     0 FILE    LOCAL  DEFAULT  ABS ./false.s
     2: 0000000000000000     0 FILE    LOCAL  DEFAULT  ABS ./rc.s
     3: 0000000000401000     0 NOTYPE  GLOBAL DEFAULT    1 _start
     4: 0000000000402001     0 NOTYPE  GLOBAL DEFAULT    2 __bss_start
     5: 0000000000402001     0 NOTYPE  GLOBAL DEFAULT    2 _edata
     6: 0000000000402008     0 NOTYPE  GLOBAL DEFAULT    2 _end
     7: 0000000000402000     0 OBJECT  GLOBAL DEFAULT    2 RC
```
As we can see the Type of the file is `EXC (Executable file)` now and as a result there is a new part in this dump compared
to the output of `rc.o`: Program Headers. The three lines under the Program Headers describes how this executable needs to 
loaded into the memory when it gets run. The first line has `R` in the `Flg` column meaning that it can only be read. The 
second line has `RE` meaning it can be read and executed. This hold the `.text` section as we defined in the assembly code 
with `section .text`. The third line shows the `.data` section which can be read and written (`Flg` = `RW`)

We can also see that the `RC` symbol was merged into the `.symtab` of this file and it points to the location `0x0000000000402000`
(last line). As in the `rc.o` file the `RC` symbol is located in the `.data` section (`Ndx = 2` meaning the index two in the
Section Headers above. One can also use the Value of the symbol (`0x0000000000402000`) and find the same address in the
Address column of the Section Headers. This means that the `RC` symbol points exactly to the byte of the `.data` section.

Since we have an executable part in our file we can dump its content with objdump like this:
```
> objdump -M intel -d ./false
0000000000401000 <_start>:
  401000:       48 8b 3c 25 00 20 40    mov    rdi,QWORD PTR ds:0x402000
  401007:       00
  401008:       b8 3c 00 00 00          mov    eax,0x3c
  40100d:       0f 05                   syscall
```
As you can see the `mov rdi,[RC]` was replaced with `mov rdi,QWORD PTR ds:0x402000`. As you can see the address in this
intruction is the same as the Value of the `RC` symbole. So it point to the same byte of the `.data` section and it will
use the value located there which is in our case 1.

Let's checkout the memory mappings of our executable in gdb
```
> gdb ./false
(gdb) break _start
(gdb) run
(gdb) info proc mappings
          Start Addr           End Addr       Size     Offset  Perms  objfile
            0x400000           0x401000     0x1000        0x0  r--p   /false
            0x401000           0x402000     0x1000     0x1000  r-xp   /false
            0x402000           0x403000     0x1000     0x2000  rw-p   /false
      0x7ffff7ff9000     0x7ffff7ffd000     0x4000        0x0  r--p   [vvar]
      0x7ffff7ffd000     0x7ffff7fff000     0x2000        0x0  r-xp   [vdso]
      0x7ffffffde000     0x7ffffffff000    0x21000        0x0  rw-p   [stack]
  0xffffffffff600000 0xffffffffff601000     0x1000        0x0  --xp   [vsyscall]
```
As you can see there are three mappings pointing to our executable with the same permission as we discussed about the
Program Headers part. These mappings are created by the kernel as it's initalizes our process. As you can see the Start 
and End Address are the same as the VirtAddr in the Program Headers. 

## Dynamic linking
As you could see in case of static linking all the code will be merged into a single executable. This makes everything
really simple but it also means that if there is two executable using the same library the code of the library will be
two times in the memory. It also takes twice as much space on the disk. Since the code is not shared it can not have the
same cache entries either. So even though the library code is exactly the same if one process loads it into the CPU cache
another needs to overwrite it resulting into constant cache misses.

Luckily there is a solution for that called shared libraries. But as always flexibility brings complexity.
Let's create a shared library from the `rc.o` and link our `false.o` dynamically against it.
Since the code of `rc.s` is dead simple, it doesn't need to be recompiled with `nasm`. But for the bigger code bases
needs to be written differently if it's mean to be a shared library. More about that later on.
To create the lib we have to link it as shared. 
```
> ld -shared rc.o -o rc.so
```
Since this will generate much more information we don't print everything with `readelf -Wa` but only the important parts with
some command line flags. All of them can be found with `readelf --help`.
```
> readelf -Wh rc.so | grep Type
Type: DYN (Shared object file)
```
As we can see in the elf header the type of this file is `DYN (Shared object file)`. 
In the Program Headers wen can see that there is no more execution part (`Flg=RE`) but there are some other types like
`DYNAMIC` and `GNU_RELRO`. TODO: Describe what are these for.
```
> readelf -Wl ./rc.so
Program Headers:
  Type      Offset   VirtAddr           PhysAddr           FileSiz  MemSiz   Flg Align
  LOAD      0x000000 0x0000000000000000 0x0000000000000000 0x001000 0x001000 R   0x1000
  LOAD      0x001f40 0x0000000000001f40 0x0000000000001f40 0x0000c1 0x0000c1 RW  0x1000
  DYNAMIC   0x001f40 0x0000000000001f40 0x0000000000001f40 0x0000c0 0x0000c0 RW  0x8
  GNU_RELRO 0x001f40 0x0000000000001f40 0x0000000000001f40 0x0000c0 0x0000c0 R   0x1
```

Let's checkout the symbols in this file
```
> readelf -Ws rc.so
Symbol table '.dynsym' contains 2 entries:
   Num:    Value          Size Type    Bind   Vis      Ndx Name
     0: 0000000000000000     0 NOTYPE  LOCAL  DEFAULT  UND
     1: 0000000000002000     0 OBJECT  GLOBAL DEFAULT    7 RC

Symbol table '.symtab' contains 5 entries:
   Num:    Value          Size Type    Bind   Vis      Ndx Name
     0: 0000000000000000     0 NOTYPE  LOCAL  DEFAULT  UND
     1: 0000000000000000     0 FILE    LOCAL  DEFAULT  ABS ./rc.s
     2: 0000000000000000     0 FILE    LOCAL  DEFAULT  ABS
     3: 0000000000001f40     0 OBJECT  LOCAL  DEFAULT    6 _DYNAMIC
     4: 0000000000002000     0 OBJECT  GLOBAL DEFAULT    7 RC
```
It looks a bit different from the one we saw in our staticly linked binary or in our object file. It has now the address of 
`0x0000000000002000` which is much smaller then the on we saw in the staticly linked binary (`0x0000000000402000`). This 
happens because it's still an intermediate address. It shows only where it is located in the shared object file. As opposed
to this in the statically linked binary it showed us a real memory address where it will be located once the code is loaded
in to the memory and the process gets run.

By static linking we have the luxury that we can expect that the code will be mapped always into the same location of the
memory (`0x0000000000400000`) and so we can calculate the absolute addresses of the symbols already at the link time.
As opposed to this the dynamically loaded libraries must expect to be loaded into a random location of the address space.
Otherwise we should have a global register about the memory addresses where the different libraries are going to be loaded.
(A bit like the public ip addresses get assigned to companies).

As a result all the symbol addreses of a shared library needsto be updated once it got loaded into the memory. That's the job
of the dynamic loader which we are going to implement in these series.

But first let's create our executable by dynamically linking against our `rc.so` library. This time we need to modify our
source code. Since the executable can only know the exact location of the library once it's got loaded we have to write
our code in a way which respects this approach
```
global _start
extern RC:data

section .text
_start:
    mov rax,[rel RC wrt ..got]
    mov rdi,[rax]
    mov rax,0x3c
    syscall
```
Let's recompile and run our command. To do that we need to find the dynamic loader of the system which can be done like this
```
> ls /lib64/ld*
/lib64/ld-linux-x86-64.so.2
```
Now we can pass it to our linker
```
> nasm -f elf64 rc.s
> ld --dynamic-linker /lib64/ld-linux-x86-64.so.2 -o false false.o -L. -l:./rc.so
> ./false; echo $?
1
```

Let's checkout the memory in gdb
```
> gdb ./false
(gdb) break _start
(gdb) run
Starting program: /home/taabodal/work/blog/code/target/false
Breakpoint 1, 0x00007ffff7fe3290 in _start () from /lib64/ld-linux-x86-64.so.2
(gdb) info proc mappings
          Start Addr           End Addr       Size     Offset  Perms  objfile
            0x400000           0x401000     0x1000        0x0  r--p   /false
            0x401000           0x402000     0x1000     0x1000  r-xp   /false
            0x402000           0x404000     0x2000     0x2000  rw-p   /false
      0x7ffff7fbd000     0x7ffff7fc1000     0x4000        0x0  r--p   [vvar]
      0x7ffff7fc1000     0x7ffff7fc3000     0x2000        0x0  r-xp   [vdso]
      0x7ffff7fc3000     0x7ffff7fc5000     0x2000        0x0  r--p   /lib/ld.so
      0x7ffff7fc5000     0x7ffff7fef000    0x2a000     0x2000  r-xp   /lib/ld.so
      0x7ffff7fef000     0x7ffff7ffa000     0xb000    0x2c000  r--p   /lib/ld.so
      0x7ffff7ffb000     0x7ffff7fff000     0x4000    0x37000  rw-p   /lib/ld.so
      0x7ffffffde000     0x7ffffffff000    0x21000        0x0  rw-p   [stack]
  0xffffffffff600000 0xffffffffff601000     0x1000        0x0  --xp   [vsyscall]
```
There are multiple things to see: Even though we break at the `_start` function of ours it stoppes at another `_start` function.
This is the one of the dynamic linker (ld.so). (Note that I rewrote name of the `ld.so` because it doesn't matter but
makes the look of the article ugly)
The other thing is to see is that compared to our static binary there is the dynamic loader also mapped into our memory
address space. And if you hit continue in the debugger, let it stop at our `_start` function and check the mappings again
you'll see that the `rc.so` is mapped to. The loading of such shared libraries at the startup of the program is one of the
jobs of the dynamic loader.
```
(gdb) continue
(gdb) info proc mappings
          Start Addr           End Addr       Size     Offset  Perms  objfile
            0x400000           0x401000     0x1000        0x0  r--p   /false
            0x401000           0x402000     0x1000     0x1000  r-xp   /false
            0x402000           0x403000     0x1000     0x2000  r--p   /false
            0x403000           0x404000     0x1000     0x3000  rw-p   /false
      0x7ffff7fb6000     0x7ffff7fb8000     0x2000        0x0  rw-p
      0x7ffff7fb8000     0x7ffff7fb9000     0x1000        0x0  r--p   /rc.so
      0x7ffff7fb9000     0x7ffff7fba000     0x1000     0x1000  r--p   /rc.so
      0x7ffff7fba000     0x7ffff7fbb000     0x1000     0x2000  rw-p   /rc.so
      0x7ffff7fbb000     0x7ffff7fbd000     0x2000        0x0  rw-p
      0x7ffff7fbd000     0x7ffff7fc1000     0x4000        0x0  r--p   [vvar]
      0x7ffff7fc1000     0x7ffff7fc3000     0x2000        0x0  r-xp   [vdso]
      0x7ffff7fc3000     0x7ffff7fc5000     0x2000        0x0  r--p   /lib/ld.so
      0x7ffff7fc5000     0x7ffff7fef000    0x2a000     0x2000  r-xp   /lib/ld.so
      0x7ffff7fef000     0x7ffff7ffa000     0xb000    0x2c000  r--p   /lib/ld.so
      0x7ffff7ffb000     0x7ffff7fff000     0x4000    0x37000  rw-p   /lib/ld.so
      0x7ffffffde000     0x7ffffffff000    0x21000        0x0  rw-p   [stack]
  0xffffffffff600000 0xffffffffff601000     0x1000        0x0  --xp   [vsyscall]
```

## Position independent executable (PIE)
As we discussed above all the shared libraries needs to be position independent, since they can be loaded anywhere in the memory.
To achive that we have to write pisition independent code (PIC) or instruct the compiler to write pic assembly for us (`gcc -fpic`).
But can we do the same for executables? Yes we can. In princip it is the same process. We need to write code that must expect
to be loaded anywhere in the memory and link it with the `-pie` flag. Since the source code of our executable is basicly empty
we can already link it as pie. A position independent executable can statically as well as dynamically linked. There is a 
restriction though. All the components which we are linking against needs to be written in a PIC way. By dynamic libraries it is
by default so, but in case of static libraries we need to rewrite or regenerate our code in a PIC way.


### Static PIE
Our `false.s` should look like this now:
```
global _start
extern RC:data

section .text
_start:
    mov rax,[rel RC wrt ..got]
    lea rdi,[rax]
    mov rax,0x3c
    syscall
```
And we can compile it like
```
> nasmf -f elf64 false.s
> ld -static -pie --no-dynamic-linker -o false false.o rc.o
```
It will change the header of the elf file
```
> readelf -Wh ./false | grep Type
Type: DYN (Position-Independent Executable file)
```
Create the `DYNAMIC` and `GNU_RELRO` program headers
```
> readelf -Wl ./false
Program Headers:
  Type      Offset   VirtAddr           PhysAddr           FileSiz  MemSiz   Flg Align
  LOAD      0x000000 0x0000000000000000 0x0000000000000000 0x0001d9 0x0001d9 R   0x1000
  LOAD      0x001000 0x0000000000001000 0x0000000000001000 0x000015 0x000015 R E 0x1000
  LOAD      0x002000 0x0000000000002000 0x0000000000002000 0x000000 0x000000 R   0x1000
  LOAD      0x002f20 0x0000000000002f20 0x0000000000002f20 0x0000e1 0x0000e1 RW  0x1000
  DYNAMIC   0x002f20 0x0000000000002f20 0x0000000000002f20 0x0000e0 0x0000e0 RW  0x8
  GNU_RELRO 0x002f20 0x0000000000002f20 0x0000000000002f20 0x0000e0 0x0000e0 R   0x1
```
And if we have a look at the mapping of the running process we can see that our executable wasn't mapped at `0x400000` anymore
but at `0x7ffff7ffb000`.
```
> gdb ./false
(gdb) break _start
(gdb) run
(gdb) info proc mappings
          Start Addr           End Addr       Size     Offset  Perms  objfile
      0x7ffff7ff5000     0x7ffff7ff9000     0x4000        0x0  r--p   [vvar]
      0x7ffff7ff9000     0x7ffff7ffb000     0x2000        0x0  r-xp   [vdso]
      0x7ffff7ffb000     0x7ffff7ffc000     0x1000        0x0  r--p   /false
      0x7ffff7ffc000     0x7ffff7ffd000     0x1000     0x1000  r-xp   /false
      0x7ffff7ffd000     0x7ffff7fff000     0x2000     0x2000  rw-p   /false
      0x7ffffffde000     0x7ffffffff000    0x21000        0x0  rw-p   [stack]
  0xffffffffff600000 0xffffffffff601000     0x1000        0x0  --xp   [vsyscall]
```

### Dynamic PIE
We can use the same `false.s` like we did in the dynamic library section and link it with
```
> ld -pie --dynamic-linker /lib64/ld-linux-x86-64.so.2 -o false false.o -L. -l:./rc.so
```
In gdb we can also see that it was mapped into the high address range:
```
> gdb ./false
(gdb) break _start
(gdb) run
(gdb) continue
(gdb) info proc mappings
          Start Addr           End Addr       Size     Offset  Perms  objfile
      0x555555554000     0x555555555000     0x1000        0x0  r--p   /false
      0x555555555000     0x555555556000     0x1000     0x1000  r-xp   /false
      0x555555556000     0x555555557000     0x1000     0x2000  r--p   /false
      0x555555557000     0x555555558000     0x1000     0x3000  rw-p   /false
      0x7ffff7fb6000     0x7ffff7fb8000     0x2000        0x0  rw-p
      0x7ffff7fb8000     0x7ffff7fb9000     0x1000        0x0  r--p   /rc.so
      0x7ffff7fb9000     0x7ffff7fba000     0x1000     0x1000  r--p   /rc.so
      0x7ffff7fba000     0x7ffff7fbb000     0x1000     0x2000  rw-p   /rc.so
      0x7ffff7fbb000     0x7ffff7fbd000     0x2000        0x0  rw-p
      0x7ffff7fbd000     0x7ffff7fc1000     0x4000        0x0  r--p   [vvar]
      0x7ffff7fc1000     0x7ffff7fc3000     0x2000        0x0  r-xp   [vdso]
      0x7ffff7fc3000     0x7ffff7fc5000     0x2000        0x0  r--p   /lib/ld.so
      0x7ffff7fc5000     0x7ffff7fef000    0x2a000     0x2000  r-xp   /lib/ld.so
      0x7ffff7fef000     0x7ffff7ffa000     0xb000    0x2c000  r--p   /lib/ld.so
      0x7ffff7ffb000     0x7ffff7fff000     0x4000    0x37000  rw-p   /lib/ld.so
      0x7ffffffde000     0x7ffffffff000    0x21000        0x0  rw-p   [stack]
  0xffffffffff600000 0xffffffffff601000     0x1000        0x0  --xp   [vsyscall]
```

There is a new section in the output of readelf which we haven't seen before: the Relocations
```
> readelf -Wr false
Relocation section '.rela.dyn' at offset 0x298 contains 1 entry:
    Offset             Info             Type               Symbol's Value  Symbol's Name + Addend
0000000000002ff8  0000000100000006 R_X86_64_GLOB_DAT      0000000000000000 RC + 0
```
As we're referencing a variable which is located in a shared library we can not the address of it before the library
will be mapped. So the linker does an indiretion for us. Instread of referencing the variable directly we are referencing
a memory address which is tied to our binary and which serves as a pointer to the real address of the variable. Hence the
assembly code `mov rax,[rel RC wrt ..got]` which could be interpreted like this:
1. Calculate a relative location of `RC` With Reference To `GOT`
2. Put the value which can be found in this location into `rax`

So what is GOT? It resolves to the Global Offset Table. GOT is a location in our program which we can use to delay the 
referencing of a value. It works a bit like a phone book. You know the name of the person you wanna call so you look the
number of it in the book and after you use that number to reach the person. It's also a bit different from the book because
it's empty at the beginning of our program.
```
> xxd -c8 false | grep 002ff8
00002ff8: 0000 0000 0000 0000  ........
```
If you look at the offset of the relocation (`0x0000000000002ff8`) and look it up in the Section Headers, you'll see that
it points to the first element of the GOT
```
> readelf -WS false | grep -E '0000000000002ff8|Name'
  [Nr] Name Type     Address          Off    Size   ES Flg Lk Inf Al
  [10] .got PROGBITS 0000000000002ff8 002ff8 000008 08  WA  0   0  8
```
Every time a pie program is started the dynamic linker will check if there is any relocations in the program which needs to be
made and if there is any, it'll fix up the addresses of the executable. It's also true for every dynamic libraries.

Let's prove this with gdb
```
> gdb ./false
(gdb) break _start
(gdb) run
(gdb) info proc mappings
          Start Addr           End Addr       Size     Offset  Perms  objfile
      0x555555554000     0x555555555000     0x1000        0x0  r--p   /false
      0x555555555000     0x555555556000     0x1000     0x1000  r-xp   /false
      0x555555556000     0x555555558000     0x2000     0x2000  rw-p   /false
      0x7ffff7fbd000     0x7ffff7fc1000     0x4000        0x0  r--p   [vvar]
      0x7ffff7fc1000     0x7ffff7fc3000     0x2000        0x0  r-xp   [vdso]
      0x7ffff7fc3000     0x7ffff7fc5000     0x2000        0x0  r--p   /lib/ld.so
      0x7ffff7fc5000     0x7ffff7fef000    0x2a000     0x2000  r-xp   /lib/ld.so
      0x7ffff7fef000     0x7ffff7ffa000     0xb000    0x2c000  r--p   /lib/ld.so
      0x7ffff7ffb000     0x7ffff7fff000     0x4000    0x37000  rw-p   /lib/ld.so
      0x7ffffffde000     0x7ffffffff000    0x21000        0x0  rw-p   [stack]
  0xffffffffff600000 0xffffffffff601000     0x1000        0x0  --xp   [vsyscall]
```
As you can see our prgram was mapped at the address of `0x555555554000`. If we add the offset of the relocation to this address
we can get the value of this memory region. At this point it is zero because the dynamic linker has just started and haven't 
done any fixings. Once we let the program continue and stop on out `_start` function the dynamic linker has already finished
it's first job and the value pointed by the relocation has been changed.
```
(gdb) x/1gx 0x555555554000 + 0x002ff8
0x555555556ff8: 0x0000000000000000

(gdb) continue

(gdb) x/1gx 0x555555554000 + 0x002ff8
0x555555556ff8: 0x00007ffff7fba000
```
At this point our program is ready to use this indirection to access the memory location of `RC`.
```
(gdb) x/1bx 0x00007ffff7fba000
0x7ffff7fba000: 0x01
```

## Conclusion
To summarize the above we could say the followings:
- **PIC**: Position independent code is a type of assembly code which only uses relative addressing. This is must for
    dynamicly linked libaries and an option for the executables.
- **PIE**: Poisition independent executable is an executable which written with PIC code only and so it can be loaded anywhere
    in the memory address sapce
- **Object file**: is a compilation unit which will be relocated during the linkage. Multiple object files can be merges into
    an archive (static library) a shared object (dynamic library) or into an executable.
- **Shared object file**: is a dynamically linked library which can be loaded anywhere in the memory because it's written in PIC
- **Static linking**: is way to combine multiple object files into a single executable
- **Dynamic linking**: is a way to tell the linker that some of the dependencies will only be available at runtime
