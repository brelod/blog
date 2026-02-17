# Memory

## The .text section
Let's create a small binary without much bloat and checkout its memory footprint
```asm
section .text
global _start
_start:
    mov rax,34
    syscall
```
All it does is providing an entry point to the process and pauses the execution by calling the `pause` system call.
We can compile, link, run and display the memory of it as follows:
```
> nasm -f elf64 main.s && ld ./main.o && strip -s ./a.out
> ./a.out & cat /proc/$!/maps
00400000-00401000                  r--p  00000000  fd:00  940117  /a.out
00401000-00402000                  r-xp  00001000  fd:00  940117  /a.out
7ffc92d30000-7ffc92d51000          rw-p  00000000  00:00  0       [stack]
7ffc92d51000-7ffc92d55000          r--p  00000000  00:00  0       [vvar]
7ffc92d55000-7ffc92d57000          r-xp  00000000  00:00  0       [vdso]
ffffffffff600000-ffffffffff601000  --xp  00000000  00:00  0       [vsyscall]
```
If you're unfamiliar with this syntax: `$!` is a bash variable and it contains the process id of the last started process.
With `&` our process goes into the background so we can use the same terminal to print the memory mappings of it which are
expressed by the kernel at the location `/proc/<pid>/maps` as a simple file.

The columns above have the following values:
1. memory address range
2. permissions (r=read, w=write, x=exec, p=private, s=shared)
3. file offset (only if the mapping is file-backed)
4. device id (major:minor)
5. inode id
6. either the file name or some human readable identifyer of the memory range

The first two lines in the mapping shows us how our binary was mapped: 
1. `00400000-00401000`: the elf header can be found in this read-only region
2. `00401000-00402000`: this is the .text section of our binary which contains the code to be executed

We can see something similar if we look at the section headers in the file too:
```
> readelf -W -S ./a.out
Section Headers:
  [Nr] Name      Type     Address          Off    Size   ES Flg Lk Inf Al
  [ 0]           NULL     0000000000000000 000000 000000 00      0   0  0
  [ 1] .text     PROGBITS 0000000000401000 001000 000007 00  AX  0   0 16
  [ 2] .shstrtab STRTAB   0000000000000000 001007 000011 00      0   0  1
```

## The .data section
Let's create another section in our binary the `.data` by adding some initialized data to it:
```asm
section .data
    db "Hello world"
section .text
global _start
_start:
    mov rax,34
    syscall
```
If we now run our program we see an extra line about the data section
```
> nasm -f elf64 main.s && ld ./main.o && strip -s ./a.out
> ./a.out & cat /proc/$!/maps
00400000-00401000                  r--p  00000000  fd:00  940117  /a.out
00401000-00402000                  r-xp  00001000  fd:00  940117  /a.out
00402000-00403000                  rw-p  00002000  fd:00  940117  /a.out
7ffd711d1000-7ffd711f2000          rw-p  00000000  00:00  0       [stack]
ffffffffff600000-ffffffffff601000  --xp  00000000  00:00  0       [vsyscall]
```
As we can see the `00402000-00403000` section is read-write enabled but it can not be executed.
```
> readelf -W -S ./a.out
Section Headers:
  [Nr] Name      Type     Address          Off    Size   ES Flg Lk Inf Al
  [ 0]           NULL     0000000000000000 000000 000000 00      0   0  0
  [ 1] .text     PROGBITS 0000000000401000 001000 000007 00  AX  0   0 16
  [ 2] .data     PROGBITS 0000000000402000 002000 00000b 00  WA  0   0  4
  [ 3] .shstrtab STRTAB   0000000000000000 00200b 000017 00      0   0  1
```

## The .rodata section
Let's create another section in our binary the `.rodata` by adding some initialized read-only data to it:
```asm
section .rodata
    db "Hello world"
section .text
global _start
_start:
    mov rax,34
    syscall
```
If we now run our program we see an extra line about the rodata section which is mapped as `r--p` now.
```
> nasm -f elf64 main.s && ld ./main.o && strip -s ./a.out
> ./a.out & cat /proc/$!/maps
00400000-00401000                  r--p  00000000  fd:00  940149  /a.out
00401000-00402000                  r-xp  00001000  fd:00  940149  /a.out
00402000-00403000                  r--p  00002000  fd:00  940149  /a.out
7ffc213e4000-7ffc21405000          rw-p  00000000  00:00  0       [stack]
7ffc21558000-7ffc2155c000          r--p  00000000  00:00  0       [vvar]
7ffc2155c000-7ffc2155e000          r-xp  00000000  00:00  0       [vdso]
ffffffffff600000-ffffffffff601000  --xp  00000000  00:00  0       [vsyscall]
```
The elf file looks like this:
```
> readelf -W -S ./a.out
Section Headers:
  [Nr] Name      Type     Address          Off    Size   ES Flg Lk Inf Al
  [ 0]           NULL     0000000000000000 000000 000000 00      0   0  0
  [ 1] .text     PROGBITS 0000000000401000 001000 000007 00  AX  0   0 16
  [ 2] .rodata   PROGBITS 0000000000402000 002000 00000b 00   A  0   0  4
  [ 3] .shstrtab STRTAB   0000000000000000 00200b 000019 00      0   0  1
```

## The .bss section
To reserve some extra space which could be used during the execution of the process we can use the `.bss` section
```asm
section .bss
    resq 1024

section .text
global _start
_start:
    mov rax,34
    syscall
```
This creates a buffer which will be initialized with zeros at the startup of the process but it doesn't take up space
in the binary itself. We can see this section maped as reas-write too right under the `.data` section. Since it's only
logically defined by the executable the new line doesn't show the relation to the elf file.
```
> nasm -f elf64 main.s && ld ./main.o && strip -s ./a.out
> ./a.out & cat /proc/$!/maps
00400000-00401000                  r--p  00000000  fd:00  940150  /a.out
00401000-00402000                  r-xp  00001000  fd:00  940150  /a.out
00403000-00405000                  rw-p  00000000  00:00  0       
7ffc17944000-7ffc17965000          rw-p  00000000  00:00  0       [stack]
7ffc179c2000-7ffc179c6000          r--p  00000000  00:00  0       [vvar]
7ffc179c6000-7ffc179c8000          r-xp  00000000  00:00  0       [vdso]
ffffffffff600000-ffffffffff601000  --xp  00000000  00:00  0       [vsyscall]
```
And the elf file:
```
> readelf -W -S ./a.out
Section Headers:
  [Nr] Name      Type     Address          Off    Size   ES Flg Lk Inf Al
  [ 0]           NULL     0000000000000000 000000 000000 00      0   0  0
  [ 1] .text     PROGBITS 0000000000401000 001000 000007 00  AX  0   0 16
  [ 2] .bss      NOBITS   0000000000402000 002000 002000 00  WA  0   0  4
  [ 3] .shstrtab STRTAB   0000000000000000 001007 000016 00      0   0  1
```

## The heap
Let's reserve another type of memory. For the heap allocation we need to ask the kernel to move the break point of the
process a bit higher. There is a system call for that called `brk()`. If it's called with 0 as argument it returns the
current break point of the process and if it's called with a valid address it will be set as the new breakpoint.
The assembly code looks like this:
```asm
section .text
global _start
_start:
    ; old =  brk(0);
    mov rdi,0x0
    mov rax,0xc
    syscall

    ; new = brk(old + 0x1000)
    add rax,0x1000
    mov rdi,rax
    mov rax,0xc
    syscall

    ; pause()
    mov rax,34
    syscall
```
If we execute, we'll see a new line again called `[heap]`. Similarly to the `.data` and `.bss` sections it is also mapped into
the low address region of the virtual address space but differently from them the size of it can be changed. It grows towards
the high memory address region.
```
> nasm -f elf64 main.s && ld ./main.o && strip -s ./a.out
> ./a.out & cat /proc/$!/maps
00400000-00401000                  r--p  00000000  fd:00  940155  /a.out
00401000-00402000                  r-xp  00001000  fd:00  940155  /a.out
009ca000-009cb000                  rw-p  00000000  00:00  0       [heap]
7ffd483c0000-7ffd483e1000          rw-p  00000000  00:00  0       [stack]
7ffd483e4000-7ffd483e8000          r--p  00000000  00:00  0       [vvar]
7ffd483e8000-7ffd483ea000          r-xp  00000000  00:00  0       [vdso]
ffffffffff600000-ffffffffff601000  --xp  00000000  00:00  0       [vsyscall]
```
The elf
```
> readelf -W -S ./a.out
Section Headers:
  [Nr] Name      Type     Address          Off    Size   ES Flg Lk Inf Al
  [ 0]           NULL     0000000000000000 000000 000000 00      0   0  0
  [ 1] .text     PROGBITS 0000000000401000 001000 000023 00  AX  0   0 16
  [ 2] .shstrtab STRTAB   0000000000000000 001023 000011 00      0   0  1
```

## The stack
We can also change the size of the stack but I don't know how....

## The vdso, vvar and vsyscall
The `v` in the name of these sections means "virtual". The vdso section is a dynamicly linked library mapped by the kernel
into the address space of the process and it allows to call some system calls with faster execution time. Since the
call of these functions doesn't require a context switch like a normal system call it can provide a significante performance
improvement to our program. Let's dump the content of it to check the available symboles. We need our pause program again:
```asm
section .text
global _start
_start:
    mov rax,34
    syscall
```
Let's check the location of the vdso on the usual way:
```
> nasm -f elf64 main.s && ld ./main.o && strip -s ./a.out
> ./a.out & pid=$!; cat /proc/$pid/maps
00400000-00401000                  r--p  00000000  fd:00  940149  /a.out
00401000-00402000                  r-xp  00001000  fd:00  940149  /a.out
00402000-00403000                  r--p  00002000  fd:00  940149  /a.out
7ffd2023d000-7ffd2025e000          rw-p  00000000  00:00  0       [stack]
7ffd203ed000-7ffd203f1000          r--p  00000000  00:00  0       [vvar]
7ffd203f1000-7ffd203f3000          r-xp  00000000  00:00  0       [vdso]
ffffffffff600000-ffffffffff601000  --xp  00000000  00:00  0       [vsyscall]
```
Once we know the start address (`0x7ffd203f1000`) and the length (`0x7ffd203f3000 - 0x7ffd203f1000`) of the vdso section we
can use the `dd` command to dump the content of it. Note that we need root access to do this.
```
sudo dd if=/proc/$pid/mem of=vdso bs=1 skip=$((0x7ffd203f1000)) count=$((0x7ffd203f3000 - 0x7ffd203f1000))
```
After that we can analyse it just like any ather shared object files:
```
> readelf -W -s ./vdso
Symbol table '.dynsym' contains 13 entries:
   Num:    Value          Size Type    Bind   Vis      Ndx Name
     0: 0000000000000000     0 NOTYPE  LOCAL  DEFAULT  UND
     1: 0000000000000c10     5 FUNC    WEAK   DEFAULT   11 clock_gettime@@LINUX_2.6
     2: 0000000000000bd0     5 FUNC    GLOBAL DEFAULT   11 __vdso_gettimeofday@@LINUX_2.6
     3: 0000000000000c20    99 FUNC    WEAK   DEFAULT   11 clock_getres@@LINUX_2.6
     4: 0000000000000c20    99 FUNC    GLOBAL DEFAULT   11 __vdso_clock_getres@@LINUX_2.6
     5: 0000000000000bd0     5 FUNC    WEAK   DEFAULT   11 gettimeofday@@LINUX_2.6
     6: 0000000000000be0    42 FUNC    GLOBAL DEFAULT   11 __vdso_time@@LINUX_2.6
     7: 0000000000000cc0   157 FUNC    GLOBAL DEFAULT   11 __vdso_sgx_enter_enclave@@LINUX_2.6
     8: 0000000000000be0    42 FUNC    WEAK   DEFAULT   11 time@@LINUX_2.6
     9: 0000000000000c10     5 FUNC    GLOBAL DEFAULT   11 __vdso_clock_gettime@@LINUX_2.6
    10: 0000000000000000     0 OBJECT  GLOBAL DEFAULT  ABS LINUX_2.6
    11: 0000000000000c90    38 FUNC    GLOBAL DEFAULT   11 __vdso_getcpu@@LINUX_2.6
    12: 0000000000000c90    38 FUNC    WEAK   DEFAULT   11 getcpu@@LINUX_2.6
```
