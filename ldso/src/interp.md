# Interpreter

Let's create a basic dynamic loader (aka interpreter) which can hand over the control to our executable.
To be able to do that we need to have information about where is the entry point of the main executable.

When the kernel loads the program it has to find out a couple of iformation about it. Although the dynamic loader could do the
same since there informations have already been parsed the kernel can simply put them onto the stack of the process and
let the dynamic loader to find them. These are the information in the auxiliary vector which we already implemented earlier.

For a quick recap let's print out the values. Let's use `ls` command to check what kind of data will be passed when it gets started.
```
> gdb /bin/ls
(gdb) break _start
(gdb) run
(gdb) info auxv
33   AT_SYSINFO_EHDR      System-supplied DSO's ELF header 0x7ffff7fc1000
51   AT_MINSIGSTKSZ       Minimum stack size for signal delivery 0xe30
16   AT_HWCAP             Machine-dependent CPU capability hints 0xf8bfbff
6    AT_PAGESZ            System page size               4096
17   AT_CLKTCK            Frequency of times()           100
3    AT_PHDR              Program headers for program    0x555555554040
4    AT_PHENT             Size of program header entry   56
5    AT_PHNUM             Number of program headers      13
7    AT_BASE              Base address of interpreter    0x7ffff7fc3000
8    AT_FLAGS             Flags                          0x0
9    AT_ENTRY             Entry point of program         0x55555555aaa0
11   AT_UID               Real user ID                   1066129479
12   AT_EUID              Effective user ID              1066129479
13   AT_GID               Real group ID                  1065878017
14   AT_EGID              Effective group ID             1065878017
23   AT_SECURE            Boolean, was exec setuid-like? 0
25   AT_RANDOM            Address of 16 random bytes     0x7fffffffec19
26   AT_HWCAP2            Extension of AT_HWCAP          0x2
31   AT_EXECFN            File name of executable        0x7fffffffefec "/usr/bin/ls"
15   AT_PLATFORM          String identifying platform    0x7fffffffec29 "x86_64"
0    AT_NULL              End of vector                  0x0
```
As we can see the entry point is marked by `AT_ENTRY` so let's find that value.
Our main function could simply look like this:
```rust
#[no_mangle]
fn main() -> u8 { 
    for aux in linux::env::auxv() {
        if let AT::AT_ENTRY(entry) = aux {
            unsafe { 
                core::arch::asm!(
                    "jmp {}", 
                    in(reg) entry,
                    options(nostack, noreturn),
                );
            }
        }
    }

    unreachable!()
}
```
We also need to rebuild our `false` binary. And when we're there we should also get rid of the compelxity of using a library.
Let's create a simple executable which exits with one.
```
global _start

section .text
_start:
    lea rdi,0x1
    mov rax,0x3c
    syscall
```
Wenn we recompile it with our rust binary as an dynamic linker we can prove the result with
```
> nasm -f elf64 false.s
> ld -pie --dynamic-linker ld.so -o false false.o

> readelf -Wl false | grep interpreter
[Requesting program interpreter: ld.so]

> ./false; echo $?
1
```
As you can see we started the `false` executable and still our rust binary got run first.
So far so good but what happens if we need to run a non-pie executable? Let's rebuild `false` with `-no-pie` option.
```
> ld -no-pie --dynamic-linker ld.so -o false false.o
> file false
false: ELF 64-bit LSB executable, x86-64, version 1 (SYSV), statically linked, not stripped
```
It will be statically linked. It looks like we do need to link against a shared library to convince the linker to create a
dynamically linked executable. So let's link against our `rc.so` even if we don't use the variable defined there anymore.
```
> ld -no-pie --dynamic-linker ld.so -o false false.o -L. -l:rc.so
> file false
false: ELF 64-bit LSB executable, x86-64, version 1 (SYSV), dynamically linked, interpreter ld.so, not stripped
```
It looks better now but if we try to run it then we see the problem
```
> ./false
panicked at Segmentation fault
```
Let find out with gdb what's the cause:
```
> gdb ./false
(gdb) b _start
(gdb) r
Starting program: /home/taabodal/work/blog/code/target/false
Cannot access memory at address 0x66c1d40f66eec178
Cannot access memory at address 0x66c1d40f66eec170
Cannot access memory at address 0x66c1d40f66eec178
Cannot access memory at address 0x66c1d40f66eec178
Cannot access memory at address 0x66c1d40f66eec170

(gdb) info proc mappings
          Start Addr           End Addr       Size     Offset  Perms  objfile
            0x400000           0x401000     0x1000        0x0  r--p   /ld.so
            0x401000           0x404000     0x3000     0x1000  r-xp   /ld.so
            0x404000           0x405000     0x1000     0x4000  r--p   /ld.so
            0x406000           0x407000     0x1000     0x5000  rw-p   /ld.so
            0x407000           0x408000     0x1000        0x0  rw-p
      0x7ffff7ff9000     0x7ffff7ffd000     0x4000        0x0  r--p   [vvar]
      0x7ffff7ffd000     0x7ffff7fff000     0x2000        0x0  r-xp   [vdso]
      0x7ffffffde000     0x7ffffffff000    0x21000        0x0  rw-p   [stack]
  0xffffffffff600000 0xffffffffff601000     0x1000        0x0  --xp   [vsyscall]
```
As you can see there is a bunch of memory address at the startup of the process and if we list the mappings there is no
executable at all. We only have the `ld.so` mapped into the address space. So what's the problem? We built a binary which
depens on the where it gets loaded but at the standard position (`0x400000`) we have already mapped our `ld.so` and it
collides with the binary it should load. We should really build our `ld.so` with `pie` so it can live together with `pie`
and `non-pie` executable in the same address space. We can do that by specifiying the link arguments of our rust binary
in the `cargo.sh` like this `-nostartfiles -pie -Wl,--no-dynamic-linker`. Once we've done that it should be mapped into a
random location and let the main executable do its job.
```
> ./false
Segmentation fault
```
Or not... But what's the problem?
```
> gdb ./false
(gdb) r
Program received signal SIGSEGV, Segmentation fault.
0x0000000000001140 in ?? ()

(gdb) backtrace
#0  0x0000000000001140 in ?? ()
#1  0x00007ffff7ff8629 in linux::__rust_main (rsp=<optimized out>) at lib.rs:63
#2  0x00007ffff7ff85c4 in linux::_start () at lib.rs:42

(gdb) up
(gdb) disassemble
Dump of assembler code for function linux::__rust_main:
   0x00007ffff7ff85e0 <+0>:  push   rbp
   0x00007ffff7ff85e1 <+1>:  mov    rbp,rsp
   0x00007ffff7ff85e4 <+4>:  mov    rax,QWORD PTR [rdi]
   0x00007ffff7ff85e7 <+7>:  lea    rax,[rdi+rax*8]
   0x00007ffff7ff85eb <+11>: add    rax,0x10
   0x00007ffff7ff85ef <+15>: mov    rcx,rax
   0x00007ffff7ff85f2 <+18>: data16 data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
   0x00007ffff7ff8600 <+32>: cmp    QWORD PTR [rcx],0x0
   0x00007ffff7ff8604 <+36>: lea    rcx,[rcx+0x8]
   0x00007ffff7ff8608 <+40>: jne    0x7ffff7ff8600 <linux::__rust_main+32>
   0x00007ffff7ff860a <+42>: add    rdi,0x8
   0x00007ffff7ff860e <+46>: mov    QWORD PTR [rip+0x59eb],rdi    # 0x7ffff7ffe000
   0x00007ffff7ff8615 <+53>: mov    QWORD PTR [rip+0x59ec],rax    # 0x7ffff7ffe008
   0x00007ffff7ff861c <+60>: mov    QWORD PTR [rip+0x59ed],rcx    # 0x7ffff7ffe010
   0x00007ffff7ff8623 <+67>: call   QWORD PTR [rip+0x598f]        # 0x7ffff7ffdfb8
=> 0x00007ffff7ff8629 <+73>: pop    rbp
   0x00007ffff7ff862a <+74>: ret
End of assembler dump.

(gdb) x/1gx 0x7ffff7ffdfb8
0x7ffff7ffdfb8: 0x0000000000001140
```
It seems like we're doing some relative addressing there and trying to jump to the location `0x0000000000001140`. 
At this address there is definitelly nothing to look for. So what's this address? Where does it come from? It seems to be
a relative relocation to somewhere. But where
```
> readelf -Wr bin | grep 1140
0000000000006fb8  0000000000000008 R_X86_64_RELATIVE                         1140

> nm --demangle=rust bin | grep 1140
0000000000001140 T main
```
That's our main function. The ld.so try to start its main function but to be able to call that it needs to be relocated first.
And who will do this relocation if there is no ld.so running? Well, there is one. We're building it right now...

To be continued...

