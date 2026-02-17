# mmap

## Memory protection
Although `brk` is a nice little tool for allocation, there quite a lot of other things we can do with memory.
To write self modifying code we can use mmap to allocate a memory which has all the read-write-exec flags enabled

Let's create an executable which can read byte stream from standard out and it tries to execute it.
```rust
#![no_std]
#![no_main]

extern crate linux;
use linux::syscall::*;

#[no_mangle]
fn main() -> u8 { 
    let ptr = unsafe { 
        mmap(
            core::ptr::null_mut(), 1024, 
            PROT_READ|PROT_WRITE|PROT_EXEC, 
            MAP_PRIVATE|MAP_ANONYMOUS, 
            0, 0
        ).unwrap() 
    };

    let mut buf = unsafe { core::slice::from_raw_parts_mut(ptr, 1024) };

    if read(0, &mut buf).unwrap() > 0 {
        unsafe { core::arch::asm!("jmp {0}", in(reg) ptr) }
    }

    0
}
```
Let's break our program down: First we need to allocate a buffer which we can fill with data
```rust
let ptr = unsafe { 
    mmap(
        core::ptr::null_mut(), 1024, 
        PROT_READ|PROT_WRITE|PROT_EXEC, 
        MAP_PRIVATE|MAP_ANONYMOUS, 
        0, 0
    ).unwrap() 
};
```
After that we create a slice to make sure that we avoid any memory safety issues... 
```rust
let mut buf = unsafe { core::slice::from_raw_parts_mut(ptr, 1024) };
```
Once we've done with that we can read data from stdin into this buffer and if there were some data
we can try to execute it.
```rust
if read(0, &mut buf).unwrap() > 0 {
    unsafe { core::arch::asm!("jmp {0}", in(reg) ptr) }
}
```
If there is no data present, we simply exit with process with return code 0.
Let's test our program like this:
```
> ./cargo.sh build

> cat /dev/null | ./target/bin; echo $?
0

> echo "hello world" | ./target/bin; echo $?
Segmentation fault (core dumped)
139
```

It seems to be working, so let's write some code which is able to rewrite itself:
```asm
global exploit
.text:
exploit:
    mov rdi,0x1
    inc byte [rel exploit + 0x1]
    cmp rdi,0xa
    jb exploit
    mov rax,0x3c
    syscall
```
This code initializes `rdi` with `0x1` and increments the constant value of `0x1` by one.
After that it checks if `rdi` is already equals to `0xa` and if not it jumps back to `exploit` but this
time we put `0x2` into `rdi`. Once the `rdi` reaches `0xa` it calls the exit system call so the return
code of our process will be 10.

Let's build that code and see how it looks after the compilation:
```
> nasm -f elf64 -o obj asm.s
> objdump --disassemble=exploit -M intel ./obj
0000000000000000 <exploit>:
   0:   bf 01 00 00 00          mov    edi,0x1
   5:   fe 05 f6 ff ff ff       inc    BYTE PTR [rip+0xfffffffffffffff6]        # 1 <exploit+0x1>
   b:   48 83 ff 0a             cmp    rdi,0xa
   f:   72 ef                   jb     0 <exploit>
  11:   b8 3c 00 00 00          mov    eax,0x3c
  16:   0f 05                   syscall
```
We can dump our exploit function as a binary blob so we can use it against our rust program like this:
```
> objcopy -O binary --only-section=.text obj exploit
> cat ./exploit | ./target/bin; echo $?
10
```

## File mappings
As we've seen in the `brk` section there are always some files mapped into the virtual address space of a process.
At least there is the binary which is being executed. In many times there are mappings here too. (Check out the mappings
of the cat command with `cat /proc/self/maps` or of your shell with `cat /proc/$$/maps`)

We can also map a regular file to the address space and use it like a permanent buffer for our program.
```rust
#![no_std]
#![no_main]

extern crate linux;
use linux::syscall::*;
use linux::constants::*;

#[no_mangle]
fn main() -> u8 { 
    let fd = open("/tmp/data", O_CREAT|O_APPEND|O_RDWR, S_IRUSR|S_IWUSR).unwrap();
    fallocate(fd, 0, 0, 1024).unwrap();
    let ptr = unsafe { 
        mmap(
            core::ptr::null_mut(), 1024, 
            PROT_READ|PROT_WRITE,
            MAP_SHARED_VALIDATE,
            fd, 0
        ).unwrap() 
    };

    let mut buf = unsafe { core::slice::from_raw_parts_mut(ptr, 1024) };
    let _ = write(1, buf).unwrap();
    let _ = read(0, &mut buf).unwrap();
    0
}
```
This way we can use it:
```
> echo "Hello old world" | ./target/bin

> cat /tmp/data
Hello old world

> echo "Hello new world" | ./target/bin
Hello old world

> cat /tmp/data
Hello new world
```

Feel free to reimplement the exploit above by mapping it into the virtual address space instead of reading from stdin.

# Shared memory
```rust
#![no_std]
#![no_main]

#[macro_use]
extern crate linux;
use linux::syscall::*;
use linux::constants::*;


#[no_mangle]
fn main() -> u8 { 
    let fd = open("/tmp/data", O_CREAT|O_TRUNC|O_RDWR, 0).unwrap();
    fallocate(fd, 0, 0, 1024).unwrap();

    let p1 = unsafe {
        mmap(
            core::ptr::null_mut(), 1024,
            PROT_READ|PROT_WRITE,
            MAP_SHARED_VALIDATE,
            fd, 0
        ).unwrap()
    };

    let p2 = unsafe {
        mmap(
            core::ptr::null_mut(), 1024,
            PROT_READ|PROT_WRITE,
            MAP_SHARED_VALIDATE,
            fd, 0
        ).unwrap()
    };

    let mut b1 = unsafe { core::slice::from_raw_parts_mut(p1, 1024) };
    let mut b2 = unsafe { core::slice::from_raw_parts_mut(p2, 1024) };

    b1[0] = 13;

    println!("b1[0] = {}", b1[0]);
    println!("b2[0] = {}", b2[0]);

    0
}
```
```
> ./cargo.sh run
b1[0] = 13
b2[0] = 13
```

# Overmap section with different protection
```rust
#![no_std]
#![no_main]

#[macro_use]
extern crate linux;
use linux::syscall::*;
use linux::constants::*;


#[no_mangle]
fn main() -> u8 { 
    let p1 = unsafe {
        mmap(
            core::ptr::null_mut(), 4096 * 3,
            PROT_READ,
            MAP_ANONYMOUS|MAP_PRIVATE,
            0, 0
        ).unwrap()
    };
    
    read(0, &mut [0u8]);

    let p2 = unsafe {
        mmap(
            p1.offset(4096), 4096,
            PROT_READ|PROT_WRITE,
            MAP_ANONYMOUS|MAP_PRIVATE|MAP_FIXED,
            0, 0
        ).unwrap()
    };

    read(0, &mut [0u8]);
    unsafe { munmap(p1, 4096 * 3).unwrap() };

    pause();

    0
}
```
