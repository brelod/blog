# Environment variables
Let's print our env like this:
```
> cat /proc/self/environ | tr '\000' '\n'
SHELL=/bin/bash
LESS=-RSF
TERM_PROGRAM_VERSION=3.2a
EDITOR=vim
....
_=/usr/bin/cat
```
We already have almost everyting to get access to the environment variables of our process.
Let's update the startup logic like this:
```rust
pub(crate) static ENVP: AtomicPtr<*const i8> = AtomicPtr::new(core::ptr::null_mut());

#[no_mangle]
fn __rust_main(rsp: *const u8) -> u8 {
    let argc = unsafe { *(rsp as *const isize) };
    let argv = unsafe { rsp.offset(8) as *mut *const i8 };
    let envp = unsafe { rsp.offset(8 + 8 + argc * 8) as *mut *const i8 };

    ARGV.store(argv, Ordering::Relaxed);
    ENVP.store(envp, Ordering::Relaxed);

    unsafe { main() }
}
```
The environment logic like this:
```rust
pub fn envp() -> Pointers {
    Pointers { 
        next: 0,
        ptrs: crate::ENVP.load(Ordering::Relaxed),
    }
}
```
The `main` function like this:
```rust
#[no_mangle]
fn main() -> u8 { 
    for arg in linux::env::envp() {
        println!("{}", arg);
    }
    0
}
```
So we can print the environment variables like this:
```
> ./cargo.sh build
> ./target/bin
SHELL=/bin/bash
LESS=-RSF
TERM_PROGRAM_VERSION=3.2a
TMUX=/tmp/tmux-1066129479/default,2230,8
EDITOR=vim
...
```
Apart from that the standard library provides a neat function called [`vars`](https://doc.rust-lang.org/std/env/fn.vars.html)
and [`var`](https://doc.rust-lang.org/std/env/fn.var.html). Let's implement those too by adding the followings to the `env.rs`:
```rust
pub struct Variables {
    ptrs: Pointers,
}

impl core::iter::Iterator for Variables {
    type Item = (&'static str, &'static str);

    fn next(&mut self) -> Option<Self::Item> {
        self.ptrs.next().map(|s| s.split_once('=')).flatten()
    }
}

pub fn vars() -> Variables {
    Variables { ptrs: envp() }
}

pub fn var(key: &str) -> Option<&'static str> {
    vars().find(|(k, _)| *k == key).map(|(_, v)| v)
}
```
After that we can update the `main` function like this:
```rust
#[no_mangle]
fn main() -> u8 { 
    println!("MYVAR={:?}", linux::env::var("MYVAR"));
    0
}
```
But this time we get an symbole error on compilation:
```
> ./cargo.sh build
  = note: /usr/bin/ld: /home/taabodal/work/blog/src/chapter-03/target/liblinux.rlib(liblinux.linux.77104c24dad4cdd3-cgu.0.rcgu.o): in function `<[A] as core::slice::cmp::SlicePartialEq<B>>::equal':
          linux.77104c24dad4cdd3-cgu.0:(.text._ZN73_$LT$$u5b$A$u5d$$u20$as$u20$core..slice..cmp..SlicePartialEq$LT$B$GT$$GT$5equal17h27d80543cacf2715E+0x38): undefined reference to `memcmp'
```
So let's implement `memcmp` by putting the following code into the `ffi.rs` module:
```rust
#[no_mangle]
unsafe fn memcmp(s1: *const u8, s2: *const u8, len: usize) -> i32 {
    for idx in 0 .. len {
        let offset = idx.try_into().unwrap();
        unsafe { 
            let b1 = s1.offset(offset).read(); 
            let b2 = s2.offset(offset).read(); 
            if b1 != b2 {
                return (b1 - b2).into();
            }
        }
    }
    0
}
```
So we can run our program like this:
```
> ./cargo.sh build

> ./target/bin
MYVAR=None

> MYVAR="hello world" ./target/bin
MYVAR=Some("hello world")
```
