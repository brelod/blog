# brk
Let's write a code like this and investigate the memory footprint of our program:
```rust
#![no_std]
#![no_main]

#[macro_use]
extern crate linux;

use linux::syscall::*;

#[no_mangle]
fn main() -> u8 { 
    println!("pid: {}", getpid().unwrap());
    let _ = pause();
    0
}
```
This small program gets the process id of our program and pauses the execution so we can checkout the memory
```
> ./cargo.sh run
pid: 1320734
```
Let's checkout the mappings in the proc file system by using the pid like this:
```
> cat /proc/1320734/maps
00400000-00401000                  r--p  00000000  fd:00  950935  /target/bin
00401000-00404000                  r-xp  00001000  fd:00  950935  /target/bin
00404000-00405000                  r--p  00004000  fd:00  950935  /target/bin
00406000-00407000                  rw-p  00005000  fd:00  950935  /target/bin
00407000-00408000                  rw-p  00000000  00:00  0       
7ffe1a300000-7ffe1a321000          rw-p  00000000  00:00  0       [stack]
7ffe1a3f2000-7ffe1a3f6000          r--p  00000000  00:00  0       [vvar]
7ffe1a3f6000-7ffe1a3f8000          r-xp  00000000  00:00  0       [vdso]
ffffffffff600000-ffffffffff601000  --xp  00000000  00:00  0       [vsyscall]
```
Let's break it down what this file tells us:
- Our binary is mapped into the low address rage with different permissions:
    - read-only
    - read-exec
    - read-only
    - read-write
- There is a middle section 
- In the high address range we have
    - stack
    - vvar
    - vdso
    - vsyscall

## Allocating memory
Let's modify our main function like this and run our program:
```rust
#[no_mangle]
fn main() -> u8 { 
    println!("pid: {}", getpid().unwrap());
    brk(brk(0) + 4096);
    let _ = pause();
    0
}
```
The mappings have been changed like this:
```
> cat /proc/1321009/maps
00400000-00401000                  r--p  00000000  fd:00  950935  /target/bin
00401000-00404000                  r-xp  00001000  fd:00  950935  /target/bin
00404000-00405000                  r--p  00004000  fd:00  950935  /target/bin
00406000-00407000                  rw-p  00005000  fd:00  950935  /target/bin
00407000-00408000                  rw-p  00000000  00:00  0       
004cd000-004ce000                  rw-p  00000000  00:00  0       [heap]
7ffc131b5000-7ffc131d6000          rw-p  00000000  00:00  0       [stack]
7ffc131ed000-7ffc131f1000          r--p  00000000  00:00  0       [vvar]
7ffc131f1000-7ffc131f3000          r-xp  00000000  00:00  0       [vdso]
ffffffffff600000-ffffffffff601000  --xp  00000000  00:00  0       [vsyscall]
```
There is a new section called `[heap]` mapped as a private region with read and write permissions (`rw-p`)
So let's use that space to to read the mappings from in the proc filesystem
```rust
#![no_std]
#![no_main]

extern crate linux;

use linux::syscall::*;
use linux::constants::*;

#[no_mangle]
fn main() -> u8 { 
    let len = 4096;
    let old = brk(0);
    let _ = brk(old + len) as *mut u8;
    let mut buf = unsafe { 
        core::slice::from_raw_parts_mut(old as *mut u8, len as usize) 
    };

    let fd = open("/proc/self/maps", O_RDONLY, 0).unwrap();
    loop {
        let len = read(fd, &mut buf).unwrap();
        let _ = write(1, &buf[..len]).unwrap();
        if len < buf.len() {
            break;
        }
    }
    0
}
```
It works like this:
```
> ./cargo.sh run
00400000-00401000                  r--p  00000000  fd:00  950935  /target/bin
00401000-00404000                  r-xp  00001000  fd:00  950935  /target/bin
00404000-00406000                  r--p  00004000  fd:00  950935  /target/bin
00406000-00407000                  rw-p  00005000  fd:00  950935  /target/bin
00407000-00408000                  rw-p  00000000  00:00  0       
017c8000-017c9000                  rw-p  00000000  00:00  0       [heap]
7ffc3c050000-7ffc3c071000          rw-p  00000000  00:00  0       [stack]
7ffc3c079000-7ffc3c07d000          r--p  00000000  00:00  0       [vvar]
7ffc3c07d000-7ffc3c07f000          r-xp  00000000  00:00  0       [vdso]
ffffffffff600000-ffffffffff601000  --xp  00000000  00:00  0       [vsyscall]
```
