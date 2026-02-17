# Alignment
Now that we have implemented a couple of handy helper methods we can go back to the question from chapter one: What
is the [`Options(nostack)`](https://doc.rust-lang.org/reference/inline-assembly.html#options) in the `_start` 
assembly block used for. Let's  put a `panic` into the main function:
```rust
#[no_mangle]
fn main() -> u8 { 
    painc!();
}
```
and execute the program
```
> ./cargo.sh build
> ./target/bin
panicked at ./bin.rs:9:5:
explicit panic
```
It all looks fine, right? But what happens if you remove the `nostack` option from the assembly block of the `_start` function?
```
> ./cargo.sh build
> ./target/bin
Segmentation fault (core dumped)
```
The process crashes with segfault. Let's analyse that in gdb:
```
gdb ./target/bin
(gdb) set disassembly-flavor intel
(gdb) run
Starting program: /home/taabodal/work/blog/src/chapter-02/target/bin

Program received signal SIGSEGV, Segmentation fault.
0x0000000000402557 in rust_begin_unwind ()

(gdb) disassemble
Dump of assembler code for function rust_begin_unwind:
   ...
   0x0000000000402552 <+50>:    movups xmm0,XMMWORD PTR [rsp+0x78]
=> 0x0000000000402557 <+55>:    movaps XMMWORD PTR [rsp+0x60],xmm0
   0x000000000040255c <+60>:    movaps xmm0,XMMWORD PTR [rsp+0x60]
   0x0000000000402561 <+65>:    movaps XMMWORD PTR [rsp+0x50],xmm0
   ...
End of assembler dump.
```
From the output above I removed some lines to make it easier to digest. The process crashes at the instruction
`movaps XMMWORD PTR [rsp+0x60],xmm0`. The line above `moveups` seems to be quiet similar but it doesn't crashes.
Let's lookup what these instructions are doing: 
- [`movaps`](https://www.felixcloutier.com/x86/movaps): Move Aligned Packed Single Precision Floating-Point Values
- [`movups`](https://www.felixcloutier.com/x86/movups): Move Unaligned Packed Single Precision Floating-Point Values

The key difference between these two is alignment of the memory address. While `movups` doesn't expect any alignment
of the memory address but the `movaps` expects that it is 16/32/64 byte aligned:
> When the source or destination operand is a memory operand, the operand must be aligned on a 16-byte (128-bit version), 
> 32-byte (VEX.256 encoded version) or 64-byte (EVEX.512 encoded version) boundary or a general-protection exception 
> (#GP) will be generated.

The instruction which crashes uses `[rsp+0x60]` as memory address. `0x60` is 16 byte aligned but what is the value 
of the `rsp` register? Let's go back to gdb and print the current value of the register with
```
(gdb) info registers rsp
rsp            0x7fffffffe828      0x7fffffffe828
```
It seems like we have found the reason: The value of `rsp` is not 16 byte aligned so `[rsp+0x60]` won't be 16 byte aligned
either which causes the processor to throw a general-protection exception.

That's all nice but if the aligment of the memory address is so important then why does the compiler not check if `rsp`
is in good state before calling `movaps`? As always the [System V ABI](https://refspecs.linuxbase.org/elf/x86_64-abi-0.99.pdf)
has the answer for this question. In the section 3.2.2 The Stack Frame it says:
> The end of the input argument area shall be aligned on a 16 byte boundary. In other words, the value (%rsp + 8) is always
> a multiple of 16 when control is transferred to the function entry point.

Since it's documented in the ABIs calling convention the compiler can asume that before a function is called the `rsp`
is 16 byte aligned. So if it doesn't do stack operation which misaligns the stack it should remain 16 byte aligned.
Let's go back gdb and check the stack alignment throught of our process:
```
> gdb ./target/bin
(gdb) set disassembly-flavor intel
(gdb) break _start
(gdb) break rust_begin_unwind
(gdb) run
Breakpoint 1, 0x0000000000402500 in _start ()

(gdb) info registers rsp
rsp            0x7fffffffe960      0x7fffffffe960

(gdb) continue
Breakpoint 2, 0x0000000000402520 in rust_begin_unwind ()

(gdb) info registers rsp
rsp            0x7fffffffe8b0      0x7fffffffe8b0

(gdb) disassemble
Dump of assembler code for function rust_begin_unwind:
=> 0x0000000000402520 <+0>:     sub    rsp,0x88
   0x0000000000402527 <+7>:     mov    QWORD PTR [rsp+0x10],rdi
   ...
```
As wee can see the `rsp` register is 16 byte aligned at the beginning of both, the `_start` and `rust_begin_unwind` functions.
The problem seems to be comming after that: the first instruction of the `rust_begin_unwind` function substract `0x88` from
the stack pointer which becomes unaligned this way. But why does it do that if it knows that `movaps` needs 16 byte alignment?

The reason for that is that the `rsp` has to be 16 byte aligned **before** the [`call`](https://www.felixcloutier.com/x86/call) 
instruction is executed. Since call instruction pushes the current value of the instruction pointer (`rip`) onto the stack,
which is 8 byte long, the compiler needs to compensate this as the first step of every function call. So the `sub rsp,0x88` should
actually make the `rsp` 16 byte aligned again, which means that it wasn't aligned at all wenn the `rust_begin_unwind` function
was started. To find out when did it get misaligned we need to go up on the stack frames and check the `rsp` register. Let's
see how do the stackframes look like:
```
(gdb) backtrace
#0  0x0000000000402520 in rust_begin_unwind ()
#1  0x0000000000401033 in core::panicking::panic_fmt () at library/core/src/panicking.rs:72
#2  0x00000000004010dc in core::panicking::panic () at library/core/src/panicking.rs:146
#3  0x000000000040128d in main ()

(gdb) up
#1  0x0000000000401033 in core::panicking::panic_fmt () at library/core/src/panicking.rs:72
72      in library/core/src/panicking.rs

(gdb) info registers rsp
rsp            0x7fffffffe8b8      0x7fffffffe8b8

(gdb) up
#2  0x00000000004010dc in core::panicking::panic () at library/core/src/panicking.rs:146
146     in library/core/src/panicking.rs

(gdb) info registers rsp
rsp            0x7fffffffe8f8      0x7fffffffe8f8

(gdb) up
#3  0x000000000040128d in main ()

(gdb) info registers rsp
rsp            0x7fffffffe948      0x7fffffffe948
```
If we go up on the stackframes we can checkout the value of registers right before `call` instruction. The bad news is that
the `rsp` seems to be misaligned already in the main function. This means that the whole code is corrupted. Since `main`
function is called from our `_start` function, let's invesigate that one:
```
> gdb ./target/bin
(gdb) set disassembly-flavor intel
(gdb) break _start
(gdb) run
(gdb) disassemble
Dump of assembler code for function _start:
=> 0x0000000000402500 <+0>:     push   rax
   0x0000000000402501 <+1>:     call   0x401270 <main>
   0x0000000000402506 <+6>:     mov    rdi,rax
   0x0000000000402509 <+9>:     mov    rax,0x3c
   0x0000000000402510 <+16>:    syscall
   0x0000000000402512 <+18>:    ud2
```
We seems to have the same construct here. The first instruction of the function `push rax` realigns the stack after that
the main will be called. The only difference is since the `_start` function is the entry point of our code it has never 
been called and as such this is the only function which is started with the stack 16 byte aligned. As a result the first
instruction which was meant to compensate the misalignment of the stack will be the reason of the misalignment of it.

So let's get back to the `options(nostack)`. The documentation says:

> The `asm!` block does not push data to the stack, or write to the stack red-zone (if supported by the target). 
> If this option is not used then the stack pointer is guaranteed to be suitably aligned (according to the target ABI) 
> for a function call.

If we compile and dump the `_start` function **with** `nostack` option enabled then we get the **working** assembly code:
```
> ./cargo.sh build
> ./cargo.sh dump _start
0000000000402500 <_start>:
  402500:       e8 6b ed ff ff          call   401270 <main>
  402505:       48 89 c7                mov    rdi,rax
  402508:       48 c7 c0 3c 00 00 00    mov    rax,0x3c
  40250f:       0f 05                   syscall
  402511:       0f 0b                   ud2
```
and **without** this option we get the **crashing** assembly code:
```
> ./cargo.sh build
> ./cargo.sh dump _start
0000000000402500 <_start>:
  402500:       50                      push   rax
  402501:       e8 6a ed ff ff          call   401270 <main>
  402506:       48 89 c7                mov    rdi,rax
  402509:       48 c7 c0 3c 00 00 00    mov    rax,0x3c
  402510:       0f 05                   syscall
  402512:       0f 0b                   ud2
```

So what's here happening, isn't it exactly the opposite of what the documentation says? And the answer is no.
The compiler guaranties the stack alignment the right way in case of a function call. But it has no knowledge about the
`_start` code section being never called. It thinks that it's a function just like any other. We can prove this
by moving the `call main` outside of the assembly block. It will generate the `push rax` even if the assembly block
has the `nostack` option enabled.
```rust
#[no_mangle]
fn _start() -> ! {
    extern "C" { fn main() -> u8; } 
    unsafe { main(); }
    unsafe {
        core::arch::asm!(
            "mov rdi,rax",
            "mov rax,0x3c",
            "syscall",
            options(nostack, noreturn),
        )
    }
}
```
```
> ./cargo.sh build
> ./cargo.sh dump _start
0000000000402500 <_start>:
  402500:       50                      push   rax
  402501:       48 8d 05 68 ed ff ff    lea    rax,[rip+0xffffffffffffed68]        # 401270 <main>
  402508:       ff d0                   call   rax
  40250a:       48 89 c7                mov    rdi,rax
  40250d:       48 c7 c0 3c 00 00 00    mov    rax,0x3c
  402514:       0f 05                   syscall
  402516:       0f 0b                   ud2
```

Now that we agreed that stack alignment is important let's make it permanent to avoid this bug in the future.
The simplest way to clean up the last 16 byte of the number is `and rsp,-0x10`. Let's add this to the beginning 
of the asm block:
```rust
#[no_mangle]
fn _start() -> ! {
    unsafe {
        core::arch::asm!(
            "and rsp,-0x10",
            "call main",
            "mov rdi,rax",
            "mov rax,0x3c",
            "syscall",
            options(nostack, noreturn),
        )
    }
}
```
Now it should work even without the `nostack` option because we the generated first instruction `push rax` will have
simply no effect on our code. It's a better to use the `and` instead of the `sub` or `pop` instruction here because 
`sub` and `pop` would remove 8 bytes in every case while the `and` instruction only modifies the `rsp` if it wasn't 
aligned.
Last but not least the System V ABI also says that the user space code is responsible for cleaning up the `rbp` register:
> The content of this register is unspecified at process initialization time,
> but the user code should mark the deepest stack frame by setting the frame
> pointer to zero.

So let's do that by adding an extra assembly line `xor rbp,rbp`.
```rust
#[no_mangle]
fn _start() -> ! {
    unsafe {
        core::arch::asm!(
            "xor rbp,rbp",
            "and rsp,-0x10",
            "call main",
            "mov rdi,rax",
            "mov rax,0x3c",
            "syscall",
            options(nostack, noreturn),
        )
    }
}
```
