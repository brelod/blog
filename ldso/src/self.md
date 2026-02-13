# Self relocation

## Obtain the vdso shared object
To get the vdso memory block we can use gdb with an arbitrary executable. The kernel will map the vdso at the beginning of
the process and we can dump this region of the memory like this
```
> gdb /bin/ls
(gdb) break _start
(gdb) run
(gdb) pipe info proc mappings | grep -E 'Addr|vdso'
          Start Addr           End Addr       Size     Offset  Perms  objfile
      0x7ffff7fc1000     0x7ffff7fc3000     0x2000        0x0  r-xp   [vdso]

(gdb) dump binary memory libvdso.so 0x7ffff7fc1000 0x7ffff7fc3000
```
This will create a file called `libvdso.so` in the current directory which we can use to link our binary against.
It need to be placed in the target directory in which all of the artefacts are located.
We'll create a simple executable which can write out the current time which was retreived from the vdso shared library.
For now let's start which simply returns 7 and call it `time.s`
```
global _start

section .text
_start:
    mov rdi,0x7
    mov rax,0x3c
    syscall
```
We can use it like this:
```
> nasm -f elf64 time.s -o target/time.o

> ld --dynamic-linker /lib64/ld-linux-x86-64.so.2 \
    target/time.o -o target/time \
    -L./target -lvdso -pie

> ./target/time; echo $?
7
```

## Handover the control

It seems to be working
Let's create 
