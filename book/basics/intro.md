# Building standalone binary

In this chapter we're going to create a standalone elf binary which only depends on the core rust library. For that
we're going through the following steps:

1. Create initial project
2. Disable the Rust standard library
3. Disable standard startup logic
4. Implement simplified startup logic
5. Implement simplified teardown logic
6. Implement a basic standard library


## Initialize a project
Since we only support the Linux platform let's call our new library `linux` as it will be an interface to the Linux kernel.
To get a deeper understanding how the Rust ecosystem works we won't use cargo at this point but write out all the commands which 
Cargo uses to build the libraries and binaries. Let's create a simple Rust binary with:
```
> echo 'fn main() {}' > bin.rs
> rustc bin.rs
> ./bin
> echo $?
0
```
The only thing our program currently does is giving back a number as return code but it's gonna be more than enough for now.

## Disable the Rust standard library
### #![no_std]
To disable the Rust standard library we have to add `#![no_std]` at the top of the source file:
```rust
#![no_std]
fn main() {}
```
If we try to rebuild the code we get the following errors:
```
> rustc bin.rs
error: `#[panic_handler]` function required, but not found
error: unwinding panics are not supported without std
```

### Panic handler
It seems like the std lib provides a panic-handler which is needed to compile the code. 
So let's implement it by adding the following lines at the end of the main.rs file:
```rust
#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
```

### Unwinding
But what should we do with the second error message: `unwinding panics are not supported without std`?
What does [unwinding](https://doc.rust-lang.org/nomicon/unwinding.html) mean? We can disable the unwinding
support by aborting the execution in case of panic. As a result we get another type of error message.
```
> rustc -C panic=abort bin.rs
error: using `fn main` requires the standard library
```

### #![no_main]
So the main function depends on the std too, but how we can start a program if there is no main function? 
Luckily the rustc gives us nice tips how we can solve this problem. We have to disable the compiler generated
main function and implement a Linux specific version of it. One can do this by adding the 
[`#![no_main]`](https://doc.rust-lang.org/reference/crates-and-source-files.html#the-no_main-attribute) attribute.
```rust
#![no_std]
#![no_main]

fn main() {}

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
```

```
> rustc -C panic=abort bin.rs
error: linking with `cc` failed: exit status: 1
  |
  = note: /usr/bin/ld: /usr/lib/gcc/x86_64-linux-gnu/11/../../../x86_64-linux-gnu/Scrt1.o: in function `_start':
          (.text+0x21): undefined reference to `__libc_start_main'
          collect2: error: ld returned 1 exit status
```
As you have probably expected it doesn't compile. (From now on I'm going to cleanup the long error messages
to only show the relevant informations to us) But more interestingly it doens't complain about the missing `main` function.
It complains about the missing `__libc_start_main` function. Which is a bit weird because we're compiling Rust and not C code.


## Disable standard startup logic
To investigate the problem let's go back to the std world and create a new binary which we can debug in gdb.
```
> echo 'fn main() {}' > std.rs
> rustc std.rs
> gdb ./std
(gdb) set backtrace past-main on
(gdb) set backtrace past-entry on
(gdb) break main
(gdb) run
(gdb) backtrace
#0  0x000055555555c320 in main ()
#1  0x00007ffff7d8fd90 in __libc_start_call_main (main=main@entry=0x55555555c320 <main>, argc=argc@entry=1, argv=argv@entry=0x7fffffffe948) at ../sysdeps/nptl/libc_start_call_main.h:58
#2  0x00007ffff7d8fe40 in __libc_start_main_impl (main=0x55555555c320 <main>, argc=1, argv=0x7fffffffe948, init=<optimized out>, fini=<optimized out>, rtld_fini=<optimized out>, stack_end=0x7fffffffe938) at ../csu/libc-start.c:392
#3  0x000055555555c155 in _start ()
```
The standard Rust binary seems to be using some libc symboles to start the main function.
There is the `_start` function which calls `__libc_start_main_impl` which calls `__libc_start_call_main` 
which calls the `main` function at the end. But do we really need these symboles? Do we need a main function at all? Or can 
we simply use the `_start` function as an entry point? Let's rewrite the code like this:
```rust
#![no_std]
#![no_main]

fn _start() {}

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
```
and try to compile the binary without the general startup logic provided by gcc
```
> rustc -C panic=abort bin.rs -C link-args='-nostartfiles -static'
> ./bin
Segmentation fault (core dumped)
```

## Implement startup logic
It look like we made a step further. We can compile our code now but we're unable to run it. To find the reason of a segfault
it's typically a good idea to run the binary in gdb.
```
> gdb ./bin
(gdb) set backtrace past-main on
(gdb) set backtrace past-entry on
(gdb) run
Starting program: /home/taabodal/work/blog/blog/src/chapter-01/bin

Program received signal SIGSEGV, Segmentation fault.
0x0000000000000000 in ?? ()
(gdb) backtrace
#0  0x0000000000000000 in ?? ()
#1  0x0000000000000001 in ?? ()
#2  0x00007fffffffebda in ?? ()
#3  0x0000000000000000 in ?? ()
```
That's not to much information, right? A bunch of zeros in the backtrace and some questionmarks... But where is the `_start` function
which we have defined? Let's try another tool to print the symboles of the executable:
```
> nm ./bin
0000000000401000 R __bss_start
0000000000401000 R _edata
0000000000401000 R _end
                 U _start
```
Okay, so it has at least some data which we can read. The nm command shows the address (column 1) the type the (column 2) 
and the name of the symbole (column 3). The `R` type means that the symbole is in the read-only data section of the binary
and `U` type means that the symbole is undefined. So the conclusion is that the `_start` function which we just added to 
the source is **undefined**. Which also explains why it doesn't show any memory address for this function.

Rust has a different philosophy about public and private function compared to other popular languages like C or Java.
In C or Java is everything public until you mark it specifically private. For example in C one can mark a function private
for a compilation unit with the `static` keyword. As opposed to this in Rust is everything private until you mark it specifically
public. So how can we make our `_start` function public? Let's decorate it with the 
[`#![no_mangle]`](https://doc.rust-lang.org/reference/abi.html#the-no_mangle-attribute) attribute. This attribute has two effects:
- Disables name mangling (more about that later)
- Makes the function public for the compilation unit
```rust
#![no_std]
#![no_main]

#[no_mangle]
fn _start() {}

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
```
After the function was exported the output of nm looks already much better 
(T means: the symbole is in the .text section of the code)
```
> nm bin
0000000000402000 T __bss_start
0000000000402000 T _edata
0000000000402000 T _end
0000000000401000 T _start
```

## Implement teardown logic
We have proved that we have the `_start` function implemented, so why does the segfault happen? Our function is empty
so it definitelly doesn't do any invalid memory access, or does it? Is our function really empty? Let's checkout the
code generated by the compiler:
```
> objdump --disassemble=_start -M intel ./bin
0000000000401000 <_start>:
  401000:       c3                      ret
```
Even though in the Rust source the `_start` function is completelly empty the compiler still generates a 
[return instruction](https://www.felixcloutier.com/x86/ret) for us. The first line of the documentation says already
what we have missed: 
> Transfers program control to a return address located on the top of the stack. 
> The address is usually placed on the stack by a CALL instruction, and the return 
> is made to the instruction that follows the CALL instruction

If the `_start` function is the first code which gets executed then there is no return value to jump to.
But what should we do if we can not return from a function?

The answer is: tell the kernel, that we're done and the process should be destroyed without executing further instructions.
We can do that by applying some assembly code in place of the `ret` instruction. Let's rewrite the `_start` function like this:
```rust
#[no_mangle]
fn _start() -> ! {
    unsafe {
        core::arch::asm!(
            "mov rax,0x3c",
            "mov rdi,0x0",
            "syscall",
            options(nostack, noreturn),
        )
    }
}
```
The compiler will generate the following assembly code for us:
```
> rustc -C panic=abort bin.rs -C link-args='-nostartfiles -static'
> objdump --disassemble=_start -M intel ./bin
0000000000401000 <_start>:
  401001:       48 c7 c0 3c 00 00 00    mov    rax,0x3c
  401008:       48 c7 c7 00 00 00 00    mov    rdi,0x0
  40100f:       0f 05                   syscall
  401011:       0f 0b                   ud2
```
The return instruction was replaced with the small code we provided and something weird. So what does these lines do?
The `mov rax,0x3c` moves the integer value 60 into the `rax` register of the CPU. This value is used by the kernel to identify the
request as `exit`. The second instruction moves the integer value 0 into the `rdi` register. This will be the return code
of our program. The `syscall` transfers the execution of the process to the kernel but since the process will be destroyed
the last instruction `ud2` will never be executed by the CPU. And it's perfect like that because the `ud2` is not a valid
x86_64 instruction. This way the compiler makes sure that if the `syscall` returns the process will fail immediatelly
with an Illegal Instruction error. This is the result of the [`options(noreturn)`](https://doc.rust-lang.org/reference/inline-assembly.html#options).
I encourage you to prove it yourself by putting the `ud2` instruction before the `syscall` 
instruction and let the process crash. It looks like this:
```
> ./bin
Illegal instruction (core dumped)
```
But if you remove the `ud2` instruction again, the execution of the binary gives you back 0 as return code:
```
> ./bin
> echo $?
0
```
And if you modify the value of the `rdi` register, let's say, to `13` it gives back 13 as return code:
```
> ./bin
> echo $?
13
```
Feel free to remove the `options(nostack)` attribute too and compare the generated assembly code with the original version.
Try to figure out why is the code generated like that. (We're getting back to that later on)



## Implement standard library
Until now we've implemented everything in a single binary but what we're aiming for a Linux specific standard library. 
So let's move most of the code into a file called linux.rs and add the call to the `main` function 
into the `_start` function. The library file look this now:
```rust
#![no_std]

#[no_mangle]
fn _start() -> ! {
    unsafe {
        core::arch::asm!(
            "call main",
            "mov rdi,rax",
            "mov rax,0x3c",
            "syscall",
            options(nostack, noreturn),
        )
    }
}

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
```
We're calling `main` at first, an as the System V ABI describes the return value of the function will be placed into
the `rax` register. We can simply move this value from `rax` to `rdi` so the kernel can use this information as a return
code of the process.  After that we write into the source of the executable something like this:
```rust
#![no_std]
#![no_main]

extern crate linux;

#[no_mangle]
fn main() -> u8 { 0 }
```
Since it's getting difficult to write out all the rustc commands let's create a build script to build
our library and our binary. The `cargo.sh` looks like this:
```bash
#!/bin/bash

clean() {
    rm -rf target
}

build() {
    mkdir -p target
    rustc -C panic=abort --crate-type=lib linux.rs -o target/liblinux.rlib
    rustc -C panic=abort -C link-args='-nostartfiles -static' -L target ./bin.rs -o target/bin
}

run() {
    build
    ./target/bin
}

case "$1" in
    clean) clean;;
    build) build;;
    run) run;;
    *) echo "Invalid argument '$1'";;
esac
```
After adding execute permissions to the mini cargo script it can be used like this:
```
> chmod +x ./cargo.sh
> ./cargo.sh run
> echo $?
0
```


