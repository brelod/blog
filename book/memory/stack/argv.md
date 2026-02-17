# The main function
Let's print our args like this:
```
cat -E -T /proc/self/cmdline | tr '\000' '\n'
cat
-E
-T
/proc/self/cmdline
```

In a C program we get the command line arguments directly from the main function like this
```c
int main(int argc, char **argv); 
```
If we also need to access the environment variables we can extend the function signature like this
```c
int main(int argc, char **argv, char **envp); 
```
Or further extend it to get auxiliary informations passed to the process like this:
```c
int main(int argc, char **argv, char **envp, auxv_t *auxv); 
```

But where does these information come from? To be able to answer this question we need to go back 
to the [System V ABI](missing-link) and read the section of **3.4.1 Initial Stack and Register State**.
It says that the stack of the process will be initialized as follows:
- Unspecified block
- Info block: the command line arguments and environment varibales are copied here
- Unspecified block
- End of auxiliary vector (null entry)
- Auxiliary vector entries (`auxv_t *auxv`)
- End of environment pointer vector (null pointer)
- Environment pointer vector entries (`char **envp`)
- End of argument pointer vector (null pointer)
- Argument pointer vector (`char **argv`)
- Argument pointer vector length (`int argc`)

The argument and environment pointer vectors are just an array of pointers pointing to the Info block of the stack.
To check the value of it we can use gdb like this:
```
> gdb --args ./target/bin --arg1 --arg2
(gdb) break _start
(gdb) run
(gdb) x/8s *(char**)($rsp + 8)
0x7fffffffebd6: "/blog/src/chapter-03/target/bin"
0x7fffffffec09: "--arg1"
0x7fffffffec10: "--arg2"
0x7fffffffec17: "SHELL=/bin/bash"
0x7fffffffec27: "LESS=-RSF"
0x7fffffffec31: "TERM_PROGRAM_VERSION=3.2a"
0x7fffffffec4b: "TMUX=/tmp/tmux-1066129479/default,2230,8"
0x7fffffffec74: "EDITOR=vim"
```
The `x` let's you examine a memory location of the program and the `/8s` specifies that 8 strings should be displayed.
The `$rsp + 8` is the location of the `char **argv` and if you cast and derefence it, you get the wanted memory location.
After the list of command line arguments we can see a list of environment variables. Feel free to play around the `x` 
command of gdb if you're unfamiliar to it. You can get the help of it like `help x`.

# Command line arguments
Let's try to implement a C like command line argument handling. The C ABI uses the `rdi`,
`rsi`, `rdx`, `rcx`, `r8` and `r9` registers to pass the arguments to a function. So to pass `argc` and `argv` to `main`
we just need to fill these registers with the values we can found on the stack. Let's rewrite our `_start` function like this:
```rust
#[no_mangle]
fn _start() -> ! {
    unsafe {
        core::arch::asm!(
            "and rsp,-16",
            "mov rdi,[rsp]",
            "lea rsi,[rsp+8]",
            "call main",
            "mov rdi,rax",
            "mov rax,0x3c",
            "syscall",
            options(nostack, noreturn),
        )
    }
}
```
If you look at the assembly code there is important difference between `argc` (`rdi`) and `argv` (`rsi`): The `argc` is
passed by value while the `argv` is passed by a reference. As such in case of `argc` we load the value pointed by `rsp`
into the `rdi` in the instruction `mov rdi,[rsp]`. As opposed to this the [`lea`](https://www.felixcloutier.com/x86/lea)
instruction instead of loading the value it just calculates the memory address at `[rsp+8]` and puts this address into `rsi`.
As a result `argc` can be interpreted as an integer value while `argv` can be interpreted as pointer to an array of strings.

We can now rewrite the `main` function like this:
```rust
#[no_mangle]
fn main(argc: usize, argv: *const *const i8) -> u8 { 
    use core::convert::TryInto;
    for offset in 0 .. argc {
        unsafe {
            let ptr = *argv.offset(offset as isize);
            println!("{}", core::ffi::CStr::from_ptr(ptr).to_str().unwrap());
        }
    }
    0
}
```
And if we try to compile we can see an almost expected error message: Missing `strlen` symbole:
```
> ./cargo.sh run
error: linking with `cc` failed: exit status: 1
  = note: /usr/bin/ld: target/bin.bin.97e806d2324bed6f-cgu.0.rcgu.o: in function `core::ffi::c_str::CStr::from_ptr':
          bin.97e806d2324bed6f-cgu.0:(.text._ZN4core3ffi5c_str4CStr8from_ptr17hac38e50840c901dfE+0xc): undefined reference to `strlen'
```
So let's add `strlen` to the `ffi` module:
```rust
#[no_mangle]
fn strlen(buf: *const u8) -> usize {
    let mut len = 0;
    while unsafe { *buf.offset(len) != 0 } {
        len += 1;
    }
    let x = len.try_into().unwrap();
    x
}
```
And now we have access to the command line arguments:
```
> ./cargo.sh build
> ./target/bin arg1 arg2
./target/bin
arg1
arg2
```

It works but this way the `main` function needs to implement an unsafe block to access the arguments. I think we can do better.
The Rust standard library provides an [`args()`](https://doc.rust-lang.org/std/env/fn.args.html) which returns an
[`Args`](https://doc.rust-lang.org/std/env/struct.Args.html) struct which implements the 
[`Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html) trait so one can iterate over the arguments without
the need of unsafe blocks. Let's take this as an example and implement our `env` module. Let's create a new file called `env.rs`
and include it into the `linux.rs` with `pub mod env;`.

To be able to do some initialization we won't call the `main` function directly from `_start` but we will implement a `__rust_main`
function and do the process initialization there. Let's do that
by modifying the `linux.rs` file like this:
```rust
extern "C" { fn main() -> u8; }

#[no_mangle]
fn _start() -> ! {
    unsafe {
        core::arch::asm!(
            "xor rbp,rbp",
            "and rsp,-16",
            "mov rdi,rsp",
            "call __rust_main",
            "mov rdi,rax",
            "mov rax,0x3c",
            "syscall",
            options(nostack, noreturn),
        )
    }
}

#[no_mangle]
fn __rust_main(rsp: isize) -> u8 {
    unsafe { main() }
}
```
Once we start writing Rust code it's really hard to get a pointer to the beginning of the stack where `argc`, `argv`, etc.
are located so we pass this pointer directly from assembly to our `__rust_main` function as an argument. The rest of the pointer
operations can be done via the Rust interface. The `main` function can be rewritten like this:
```rust
#[no_mangle]
fn main() -> u8 { 0 }
```
Let's add the logic to store the pointer of `argv` which can be later used to implement the `env::args()` funciton.
```rust
use core::sync::atomic::{AtomicPtr, Ordering};
pub(crate) static ARGV: AtomicPtr<*const i8> = AtomicPtr::new(core::ptr::null_mut());

#[no_mangle]
fn __rust_main(rsp: *const u8) -> u8 {
    let argv = unsafe { rsp.offset(8) as *mut *const i8 };
    ARGV.store(argv, Ordering::Relaxed);
    unsafe { main() }
}
```
The `env.rs` looks like this:
```rust
use core::ffi::CStr;
use core::sync::atomic::Ordering;

pub struct Pointers {
    next: isize,
    ptrs: *const *const i8,
}

impl core::iter::Iterator for Pointers {
    type Item = &'static str;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let ptr = *self.ptrs.offset(self.next);
            self.next += 1;
            match ptr.is_null() {
                true => None,
                false => CStr::from_ptr(ptr).to_str().ok()
            }
        }
    }
}

pub fn args() -> Pointers {
    Pointers { 
        next: 0,
        ptrs: crate::ARGV.load(Ordering::Relaxed),
    }
}
```
And we can reimplement the main function as follows:
```rust
#[no_mangle]
fn main() -> u8 { 
    for arg in linux::env::args() {
        println!("{}", arg);
    }
    0
}
```
Run the program like this:
```
> ./cargo.sh build
> ./target/bin  a1 a2
./target/bin
a1
a2
```
