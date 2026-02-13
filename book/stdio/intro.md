# Implementing standard streams

In this chapter we're going to continue builind the Linux standard library by going through the following steps:

1. Syscalls in general
2. Implement read and write syscalls
3. Make syscalls safe
4. Make syscalls idiomatic
5. Abstract standard streams
6. Implement string formatting


## Syscalls in general
In chapter one we already implemented a systemcall called `exit`. We didn't talk much about how it works. Since systemcalls
are the foundation of the communication between the user and kernel space we will implement a couple of them throughout the
following chapters. As result it's important to get a basic understanding how they work.

Systemcalls work quite similar to function calls in the sinn that a couple of registers will be upated with some data, the
execution of the current code will be interrupted to call another code section. This other code will use the values of the
registers, do some operation with them and wenn it finishes the execution returns back to the original point to the caller
function can continue with the result of the call. An important difference though is that by calling the syscall a contex
switch will occur. This means that instead of simply jumping to another code segment of the same executable the process
will be interrupted the CPU will switch to kernel mode and the code of the kernel continue to execute. The same happens at
the end of the systemcall: the CPU switches back to user-mode and continues to execute the user-space code. To tell the CPU
to make contex switches there are two instructions on x86 family called [syscall](https://www.felixcloutier.com/x86/syscall) 
and [sysret](https://www.felixcloutier.com/x86/sysret). The first is used by user-space codes to switch to kernel 
and the second is used by the kernel to switch back to user-mode.

There are many systemcalls defined by the Linux kernel. The id of these systemcalls can be found in the kernes source tree.
The 64 bit version of the x86 architecture can be found for example [here](https://github.com/torvalds/linux/blob/v6.9/arch/x86/entry/syscalls/syscall_64.tbl)

If you have already done some lower level programming (for example C/C++) you most likely already know some of these calls.
The standard C library warps these systemcalls into simple functions so you can call them in youre code without even 
realizing that a contex switch is needed. Some famous examples are the following:

- read
- write
- open
- close
- socket
- connect
- accept
- exit

Since we don't use the standard C library we need to implement these wrappers in rust to be able to use them in our binaries.

To be able to pass arguments to the kernel we need to specific registers. The question is which register should we use?
The references which describe how a binary code needs to be implemented / interpeted called Application Binary Interface (ABI).
Linux uses the [System V ABI specification](https://refspecs.linuxbase.org/elf/x86_64-abi-0.99.pdf). There are many interesting
stuff to read about in this PDF but the most important part now for us are the calling conventions. It turns out the the function
calling convention of the C language and the syscall interface are not the same. While the function arguments are passed in the
`rdi`, `rsi`, `rdx`, `rcx`, `r8`, `r9` registers the syscall interface uses the `rdi`, `rsi`, `rdx`, `r10`, `r8` and `r9` 
registers. Appart from that it's important that the `rax` register is used to pass the syscall id and to retrieve the result 
of the syscall. To conform to these requirements we can [implement a macro](https://doc.rust-lang.org/reference/macros.html) 
to provide a simple way of starting a syscall.  Let's create a file called `syscalls.rs` and add a `pub mod syscalls` to the `linux.rs` file.
```rust
macro_rules! syscall {
    ($rax:expr) => {{
        core::arch::asm!(
            "syscall",
            inout("rax") $rax,
        );
        $rax
    }};

    ($rax:expr, $rdi:expr) => {{
        let mut rax: isize;
        core::arch::asm!(
            "syscall",
            inlateout("rax") $rax => rax,
            in("rdi") $rdi,
        );
        rax
    }};

    ($rax:expr, $rdi:expr, $rsi:expr) => {{
        let mut rax: isize;
        core::arch::asm!(
            "syscall",
            inlateout("rax") $rax => rax,
            in("rdi") $rdi,
            in("rsi") $rsi,
        );
        rax
    }};

    ($rax:expr, $rdi:expr, $rsi:expr, $rdx:expr) => {{
        let mut rax: isize;
        core::arch::asm!(
            "syscall",
            inlateout("rax") $rax => rax,
            in("rdi") $rdi,
            in("rsi") $rsi,
            in("rdx") $rdx,
        );
        rax
    }};

    ($rax:expr, $rdi:expr, $rsi:expr, $rdx:expr, $r10:expr) => {{
        let mut rax: isize;
        core::arch::asm!(
            "syscall",
            inlateout("rax") $rax => rax,
            in("rdi") $rdi,
            in("rsi") $rsi,
            in("rdx") $rdx,
            in("r10") $r10,
        );
        rax
    }};

    ($rax:expr, $rdi:expr, $rsi:expr, $rdx:expr, $r10:expr, $r8:expr) => {{
        let mut rax: isize;
        core::arch::asm!(
            "syscall",
            inlateout("rax") $rax => rax,
            in("rdi") $rdi,
            in("rsi") $rsi,
            in("rdx") $rdx,
            in("r10") $r10,
            in("r8") $r8,
        );
        rax
    }};

    ($rax:expr, $rdi:expr, $rsi:expr, $rdx:expr, $r10:expr, $r8:expr, $r9:expr) => {{
        let mut rax: isize;
        core::arch::asm!(
            "syscall",
            inlateout("rax") $rax => rax,
            in("rdi") $rdi,
            in("rsi") $rsi,
            in("rdx") $rdx,
            in("r10") $r10,
            in("r8") $r8,
            in("r9") $r9,
        );
        rax
    }};
}
```
This macro can be called with variadic (1-7) number of arguments which will be passed into the specified registers.
After the registers were filled with the data the `syscall` instruction will be executed to hand over the execution
to the kernel. Note that the `asm` macro of the rust core library requires the parameters to be placed after the
assembly code it self even though they will be set before the execution.

## Read, write, exit
The simplest way to lookup how the standard C library has implemented a systemcall wrapper is to check out the manual page of the it.
For example: 
[man read.2](https://man7.org/linux/man-pages/man2/read.2.html),
[man write.2](https://man7.org/linux/man-pages/man2/write.2.html),
[man exit.2](https://man7.org/linux/man-pages/man2/exit.2.html)

The function signatures written in C look like this
```c
ssize_t read(int fd, void *buf, size_t count);
ssize_t write(int fd, const void *buf, size_t count);
void exit(int rc);
```
Let's update our `syscalls.rs` file with the following functions:

```rust
const SYS_READ: isize = 0;
const SYS_WRITE: isize = 1;
const SYS_EXIT: isize = 60;

pub fn read(fd: i32, buf: *mut u8, count: usize) -> isize {
    unsafe { syscall!(SYS_READ, fd, buf, count) }
}

pub fn write(fd: i32, buf: *const u8, count: usize) -> isize {
    unsafe { syscall!(SYS_WRITE, fd, buf, count) }
}

pub fn exit(rc: u8) -> ! {
    unsafe { syscall!(SYS_EXIT, rc as u32); }
    unreachable!();
}
```

This allows us to read some user input and write it to the stdout as follows:
```rust
#[no_mangle]
fn main() { 
    let mut buf = [0u8;1024];
    let ptr = &mut buf as *mut u8;
    linux::syscall::read(0, ptr, buf.len());
    linux::syscall::write(1, ptr, buf.len());
    0
}
```

But we have still a problem... Our program doesn't compile anymore. We have just introduced an undefined reference
```
> ./cargo.sh build
error: linking with `cc` failed: exit status: 1
  /blog/chapter-02/src/bin.rs:7: undefined reference to `memset'
```
It turns out that to be able to use the rust syntax `let buf = [0u8;1024]` the core library needs the memset symbole. This
makes sense since this expression fills up a memory region with 1024 zeros.
There are a couple symboles the core library needs to be able to work. These are typically provided by the standard C library but
since we have disabled any libraries apart from the core lib we have to implement them manually. 
The [documentation](https://doc.rust-lang.org/core/) says the expected symboles are:

- memcpy
- memmove
- memset
- memcmp
- bcmp
- strlen

There are some other expected symboles like `rust_begin_panic` and `rust_eh_personality` but we will only implement these
step by step to be able to explore which functionality of the core library needs them. Let's implement the `memset` for now
in the ffi module. We need to add a `pub mod ffi;` in to the `linux.rs` file and create `ffi.rs` with the content:
```rust
use core::convert::TryInto;

#[no_mangle]
fn memset(buffer: *mut u8, byte: u8, len: usize) -> *mut u8 {
    for idx in 0 .. len {
        let offset = idx.try_into().unwrap();
        unsafe { buffer.offset(offset).write(byte); }
    }
    buffer
}
```
And recompile the code
```
> echo "hello world" | ./cargo.sh run
hello world
```

## Safe syscalls
Wenn we write unsafe code we sign a contract with the compiler that our code is never going to be unsound. In the Rust world
a codeblock is known to sound if it can **never** cause undefined behaviour. Luckily it's quiet well defined
what "undefined" means. There a [list of actions](https://doc.rust-lang.org/reference/behavior-considered-undefined.html) which 
causes undefined behavior and if we can be sure you are not hitting any of the items of list our code in said to be sound.
Even if this list is quite straitforward it's easy to miss some small detail just like we did in the previous paragraphs.
Our code look good, right? It has basically the same signature like the C functions and it passes all the arguments to the kernel.
It doesn't do something like dereferencing raw pointers, it doesn't do array indexing, doesn't free up memory, so what could 
go wrong then? Well let's rewrite the `main` function and see what happens.
```rust
#[no_mangle]
fn main() -> u8 { 
    linux::syscall::write(1, b"X" as *const u8, 1024);
    0
}
```
If we run this code we just experience undefined behaviour: We pass the kernel a one byte length array and a length paramter 1024.
As a result it tries to write 1024 bytes after the position of our byte array and it is absolutelly not defined what will
happen in such a scenario. In our case since the byte array was in the read only section of the binary it picks up the bytes
from there.
```
./target/bin
xinternal error: entered unreachable codesyscall.rsHhzRx
A                                                          C
UAC
```
The conclusion is that Rust is only safe if every part of the code is known to be sound. Our code is not sound because the
safe rust code can pass such parameters to it which causes undefined behaviour. Let's fix that by utilizing a primitive
type in the Rust core library called [`slice`](https://doc.rust-lang.org/core/primitive.slice.html). Since the slice
bundles the buffer and its length a user of our code can not pass a length paramter which is bigger than the size of the slice.
To be more precise it can pass to our function a slice which is has an invalid length parameter but to create this slice
one need to use an other unsafe block and the auther of this unsafe block has signed the same contract with the compiler, that
it can never produce undefined behaviour. So you see the point. If all the unsafe blocks are sound then the whole language is
safe. But if any of these block is unsound the whole ecosystem is corrupted. So let's be causios with unsafe blocks.
Here is a fix for our syscalls:
```rust
pub fn read(fd: u32, buf: &mut [u8]) -> isize {
    unsafe { syscall!(SYS_READ, fd, buf.as_ptr(), buf.len()) }
}

pub fn write(fd: u32, buf: &[u8]) -> isize {
    unsafe { syscall!(SYS_WRITE, fd, buf.as_ptr(), buf.len()) }
}
```
The `main` function works like this:
```rust
#[no_mangle]
fn main() -> u8 { 
    linux::syscall::write(1, b"x");
    0
}
```
Since there is no way to missuse this syscall if you run it, it will write exaclty one character to the screen:
```
> ./cargo.sh run
x
```

## Idiomatic syscall
Although our code is now safe it is still not really idiomatic. In C programming it's normal to
return with a number wich represents the result of the function. For example all of our syscalls return with a negativ
integer in case of an error. But in rust we have a nicer way to handle error which is based on the 
[`Result`](https://doc.rust-lang.org/core/result/enum.Result.html) enum. Let's create an `Error` enum and a `Result` enum
to represent the result of our syscalls. The list of the error codes that a syscall may return can be found 
in [errno-base.h](https://github.com/torvalds/linux/blob/master/include/uapi/asm-generic/errno-base.h) and
[errno.h](https://github.com/torvalds/linux/blob/master/include/uapi/asm-generic/errno.h) After combining the content of these
files we can build a huge enum which represents these error codes
```rust
use core::fmt;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    EPERM = 1,
    ENOENT = 2,
    ESRCH = 3,
    EINTR = 4,
    EIO = 5,
    ENXIO = 6,
    E2BIG = 7,
    ENOEXEC = 8,
    EBADF = 9,
    ECHILD = 10,
    EAGAIN = 11,
    ENOMEM = 12,
    EACCES = 13,
    EFAULT = 14,
    ENOTBLK = 15,
    EBUSY = 16,
    EEXIST = 17,
    EXDEV = 18,
    ENODEV = 19,
    ENOTDIR = 20,
    EISDIR = 21,
    EINVAL = 22,
    ENFILE = 23,
    EMFILE = 24,
    ENOTTY = 25,
    ETXTBSY = 26,
    EFBIG = 27,
    ENOSPC = 28,
    ESPIPE = 29,
    EROFS = 30,
    EMLINK = 31,
    EPIPE = 32,
    EDOM = 33,
    ERANGE = 34,
    EDEADLK = 35,
    ENAMETOOLONG = 36,
    ENOLCK = 37,
    ENOSYS = 38,
    ENOTEMPTY = 39,
    ELOOP = 40,
    EWOULDBLOCK = 41,
    ENOMSG = 42,
    EIDRM = 43,
    ECHRNG = 44,
    EL2NSYNC = 45,
    EL3HLT = 46,
    EL3RST = 47,
    ELNRNG = 48,
    EUNATCH = 49,
    ENOCSI = 50,
    EL2HLT = 51,
    EBADE = 52,
    EBADR = 53,
    EXFULL = 54,
    ENOANO = 55,
    EBADRQC = 56,
    EBADSLT = 57,
    EDEADLOCK = 58,
    EBFONT = 59,
    ENOSTR = 60,
    ENODATA = 61,
    ETIME = 62,
    ENOSR = 63,
    ENONET = 64,
    ENOPKG = 65,
    EREMOTE = 66,
    ENOLINK = 67,
    EADV = 68,
    ESRMNT = 69,
    ECOMM = 70,
    EPROTO = 71,
    EMULTIHOP = 72,
    EDOTDOT = 73,
    EBADMSG = 74,
    EOVERFLOW = 75,
    ENOTUNIQ = 76,
    EBADFD = 77,
    EREMCHG = 78,
    ELIBACC = 79,
    ELIBBAD = 80,
    ELIBSCN = 81,
    ELIBMAX = 82,
    ELIBEXEC = 83,
    EILSEQ = 84,
    ERESTART = 85,
    ESTRPIPE = 86,
    EUSERS = 87,
    ENOTSOCK = 88,
    EDESTADDRREQ = 89,
    EMSGSIZE = 90,
    EPROTOTYPE = 91,
    ENOPROTOOPT = 92,
    EPROTONOSUPPORT = 93,
    ESOCKTNOSUPPORT = 94,
    EOPNOTSUPP = 95,
    EPFNOSUPPORT = 96,
    EAFNOSUPPORT = 97,
    EADDRINUSE = 98,
    EADDRNOTAVAIL = 99,
    ENETDOWN = 100,
    ENETUNREACH = 101,
    ENETRESET = 102,
    ECONNABORTED = 103,
    ECONNRESET = 104,
    ENOBUFS = 105,
    EISCONN = 106,
    ENOTCONN = 107,
    ESHUTDOWN = 108,
    ETOOMANYREFS = 109,
    ETIMEDOUT = 110,
    ECONNREFUSED = 111,
    EHOSTDOWN = 112,
    EHOSTUNREACH = 113,
    EALREADY = 114,
    EINPROGRESS = 115,
    ESTALE = 116,
    EUCLEAN = 117,
    ENOTNAM = 118,
    ENAVAIL = 119,
    EISNAM = 120,
    EREMOTEIO = 121,
    EDQUOT = 122,
    ENOMEDIUM = 123,
    EMEDIUMTYPE	= 124,
    ECANCELED = 125,
    ENOKEY = 126,
    EKEYEXPIRED	= 127,
    EKEYREVOKED	= 128,
    EKEYREJECTED = 129,
    EOWNERDEAD = 130,
    ENOTRECOVERABLE = 131,
    ERFKILL = 132,
    EHWPOISON = 133,
}


impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<Error> for isize {
    fn from(error: Error) -> Self {
        match error {
            Error::EPERM => 1,
            Error::ENOENT => 2,
            Error::ESRCH => 3,
            Error::EINTR => 4,
            Error::EIO => 5,
            Error::ENXIO => 6,
            Error::E2BIG => 7,
            Error::ENOEXEC => 8,
            Error::EBADF => 9,
            Error::ECHILD => 10,
            Error::EAGAIN => 11,
            Error::ENOMEM => 12,
            Error::EACCES => 13,
            Error::EFAULT => 14,
            Error::ENOTBLK => 15,
            Error::EBUSY => 16,
            Error::EEXIST => 17,
            Error::EXDEV => 18,
            Error::ENODEV => 19,
            Error::ENOTDIR => 20,
            Error::EISDIR => 21,
            Error::EINVAL => 22,
            Error::ENFILE => 23,
            Error::EMFILE => 24,
            Error::ENOTTY => 25,
            Error::ETXTBSY => 26,
            Error::EFBIG => 27,
            Error::ENOSPC => 28,
            Error::ESPIPE => 29,
            Error::EROFS => 30,
            Error::EMLINK => 31,
            Error::EPIPE => 32,
            Error::EDOM => 33,
            Error::ERANGE => 34,
            Error::EDEADLK => 35,
            Error::ENAMETOOLONG => 36,
            Error::ENOLCK => 37,
            Error::ENOSYS => 38,
            Error::ENOTEMPTY => 39,
            Error::ELOOP => 40,
            Error::EWOULDBLOCK => 41,
            Error::ENOMSG => 42,
            Error::EIDRM => 43,
            Error::ECHRNG => 44,
            Error::EL2NSYNC => 45,
            Error::EL3HLT => 46,
            Error::EL3RST => 47,
            Error::ELNRNG => 48,
            Error::EUNATCH => 49,
            Error::ENOCSI => 50,
            Error::EL2HLT => 51,
            Error::EBADE => 52,
            Error::EBADR => 53,
            Error::EXFULL => 54,
            Error::ENOANO => 55,
            Error::EBADRQC => 56,
            Error::EBADSLT => 57,
            Error::EDEADLOCK => 58,
            Error::EBFONT => 59,
            Error::ENOSTR => 60,
            Error::ENODATA => 61,
            Error::ETIME => 62,
            Error::ENOSR => 63,
            Error::ENONET => 64,
            Error::ENOPKG => 65,
            Error::EREMOTE => 66,
            Error::ENOLINK => 67,
            Error::EADV => 68,
            Error::ESRMNT => 69,
            Error::ECOMM => 70,
            Error::EPROTO => 71,
            Error::EMULTIHOP => 72,
            Error::EDOTDOT => 73,
            Error::EBADMSG => 74,
            Error::EOVERFLOW => 75,
            Error::ENOTUNIQ => 76,
            Error::EBADFD => 77,
            Error::EREMCHG => 78,
            Error::ELIBACC => 79,
            Error::ELIBBAD => 80,
            Error::ELIBSCN => 81,
            Error::ELIBMAX => 82,
            Error::ELIBEXEC => 83,
            Error::EILSEQ => 84,
            Error::ERESTART => 85,
            Error::ESTRPIPE => 86,
            Error::EUSERS => 87,
            Error::ENOTSOCK => 88,
            Error::EDESTADDRREQ => 89,
            Error::EMSGSIZE => 90,
            Error::EPROTOTYPE => 91,
            Error::ENOPROTOOPT => 92,
            Error::EPROTONOSUPPORT => 93,
            Error::ESOCKTNOSUPPORT => 94,
            Error::EOPNOTSUPP => 95,
            Error::EPFNOSUPPORT => 96,
            Error::EAFNOSUPPORT => 97,
            Error::EADDRINUSE => 98,
            Error::EADDRNOTAVAIL => 99,
            Error::ENETDOWN => 100,
            Error::ENETUNREACH => 101,
            Error::ENETRESET => 102,
            Error::ECONNABORTED => 103,
            Error::ECONNRESET => 104,
            Error::ENOBUFS => 105,
            Error::EISCONN => 106,
            Error::ENOTCONN => 107,
            Error::ESHUTDOWN => 108,
            Error::ETOOMANYREFS => 109,
            Error::ETIMEDOUT => 110,
            Error::ECONNREFUSED => 111,
            Error::EHOSTDOWN => 112,
            Error::EHOSTUNREACH => 113,
            Error::EALREADY => 114,
            Error::EINPROGRESS => 115,
            Error::ESTALE => 116,
            Error::EUCLEAN => 117,
            Error::ENOTNAM => 118,
            Error::ENAVAIL => 119,
            Error::EISNAM => 120,
            Error::EREMOTEIO => 121,
            Error::EDQUOT => 122,
            Error::ENOMEDIUM => 123,
            Error::EMEDIUMTYPE => 124,
            Error::ECANCELED => 125,
            Error::ENOKEY => 126,
            Error::EKEYEXPIRED => 127,
            Error::EKEYREVOKED => 128,
            Error::EKEYREJECTED => 129,
            Error::EOWNERDEAD => 130,
            Error::ENOTRECOVERABLE => 131,
            Error::ERFKILL => 132,
            Error::EHWPOISON => 133,
        }
    }
}

impl From<isize> for Error {
    fn from(number: isize) -> Self {
        match number {
            1 => Self::EPERM,
            2 => Self::ENOENT,
            3 => Self::ESRCH,
            4 => Self::EINTR,
            5 => Self::EIO,
            6 => Self::ENXIO,
            7 => Self::E2BIG,
            8 => Self::ENOEXEC,
            9 => Self::EBADF,
            10 => Self::ECHILD,
            11 => Self::EAGAIN,
            12 => Self::ENOMEM,
            13 => Self::EACCES,
            14 => Self::EFAULT,
            15 => Self::ENOTBLK,
            16 => Self::EBUSY,
            17 => Self::EEXIST,
            18 => Self::EXDEV,
            19 => Self::ENODEV,
            20 => Self::ENOTDIR,
            21 => Self::EISDIR,
            22 => Self::EINVAL,
            23 => Self::ENFILE,
            24 => Self::EMFILE,
            25 => Self::ENOTTY,
            26 => Self::ETXTBSY,
            27 => Self::EFBIG,
            28 => Self::ENOSPC,
            29 => Self::ESPIPE,
            30 => Self::EROFS,
            31 => Self::EMLINK,
            32 => Self::EPIPE,
            33 => Self::EDOM,
            34 => Self::ERANGE,
            35 => Self::EDEADLK,
            36 => Self::ENAMETOOLONG,
            37 => Self::ENOLCK,
            38 => Self::ENOSYS,
            39 => Self::ENOTEMPTY,
            40 => Self::ELOOP,
            41 => Self::EWOULDBLOCK,
            42 => Self::ENOMSG,
            43 => Self::EIDRM,
            44 => Self::ECHRNG,
            45 => Self::EL2NSYNC,
            46 => Self::EL3HLT,
            47 => Self::EL3RST,
            48 => Self::ELNRNG,
            49 => Self::EUNATCH,
            50 => Self::ENOCSI,
            51 => Self::EL2HLT,
            52 => Self::EBADE,
            53 => Self::EBADR,
            54 => Self::EXFULL,
            55 => Self::ENOANO,
            56 => Self::EBADRQC,
            57 => Self::EBADSLT,
            58 => Self::EDEADLOCK,
            59 => Self::EBFONT,
            60 => Self::ENOSTR,
            61 => Self::ENODATA,
            62 => Self::ETIME,
            63 => Self::ENOSR,
            64 => Self::ENONET,
            65 => Self::ENOPKG,
            66 => Self::EREMOTE,
            67 => Self::ENOLINK,
            68 => Self::EADV,
            69 => Self::ESRMNT,
            70 => Self::ECOMM,
            71 => Self::EPROTO,
            72 => Self::EMULTIHOP,
            73 => Self::EDOTDOT,
            74 => Self::EBADMSG,
            75 => Self::EOVERFLOW,
            76 => Self::ENOTUNIQ,
            77 => Self::EBADFD,
            78 => Self::EREMCHG,
            79 => Self::ELIBACC,
            80 => Self::ELIBBAD,
            81 => Self::ELIBSCN,
            82 => Self::ELIBMAX,
            83 => Self::ELIBEXEC,
            84 => Self::EILSEQ,
            85 => Self::ERESTART,
            86 => Self::ESTRPIPE,
            87 => Self::EUSERS,
            88 => Self::ENOTSOCK,
            89 => Self::EDESTADDRREQ,
            90 => Self::EMSGSIZE,
            91 => Self::EPROTOTYPE,
            92 => Self::ENOPROTOOPT,
            93 => Self::EPROTONOSUPPORT,
            94 => Self::ESOCKTNOSUPPORT,
            95 => Self::EOPNOTSUPP,
            96 => Self::EPFNOSUPPORT,
            97 => Self::EAFNOSUPPORT,
            98 => Self::EADDRINUSE,
            99 => Self::EADDRNOTAVAIL,
            100 => Self::ENETDOWN,
            101 => Self::ENETUNREACH,
            102 => Self::ENETRESET,
            103 => Self::ECONNABORTED,
            104 => Self::ECONNRESET,
            105 => Self::ENOBUFS,
            106 => Self::EISCONN,
            107 => Self::ENOTCONN,
            108 => Self::ESHUTDOWN,
            109 => Self::ETOOMANYREFS,
            110 => Self::ETIMEDOUT,
            111 => Self::ECONNREFUSED,
            112 => Self::EHOSTDOWN,
            113 => Self::EHOSTUNREACH,
            114 => Self::EALREADY,
            115 => Self::EINPROGRESS,
            116 => Self::ESTALE,
            117 => Self::EUCLEAN,
            118 => Self::ENOTNAM,
            119 => Self::ENAVAIL,
            120 => Self::EISNAM,
            121 => Self::EREMOTEIO,
            122 => Self::EDQUOT,
            123 => Self::ENOMEDIUM,
            124 => Self::EMEDIUMTYPE,
            125 => Self::ECANCELED,
            126 => Self::ENOKEY,
            127 => Self::EKEYEXPIRED,
            128 => Self::EKEYREVOKED,
            129 => Self::EKEYREJECTED,
            130 => Self::EOWNERDEAD,
            131 => Self::ENOTRECOVERABLE,
            132 => Self::ERFKILL,
            133 => Self::EHWPOISON,
            other => panic!("Invalid error code: {}", other),
        }
    }
}

impl Error {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::EPERM => "Operation not permitted",
            Self::ENOENT => "No such file or directory",
            Self::ESRCH => "No such process",
            Self::EINTR => "Interrupted system call",
            Self::EIO => "I/O error",
            Self::ENXIO => "No such device or address",
            Self::E2BIG => "Arg list too long",
            Self::ENOEXEC => "Exec format error",
            Self::EBADF => "Bad file number",
            Self::ECHILD => "No child processes",
            Self::EAGAIN => "Try again",
            Self::ENOMEM => "Out of memory",
            Self::EACCES => "Permission denied",
            Self::EFAULT => "Bad address",
            Self::ENOTBLK => "Block device required",
            Self::EBUSY => "Device or resource busy",
            Self::EEXIST => "File exists",
            Self::EXDEV => "Cross-device link",
            Self::ENODEV => "No such device",
            Self::ENOTDIR => "Not a directory",
            Self::EISDIR => "Is a directory",
            Self::EINVAL => "Invalid argument",
            Self::ENFILE => "File table overflow",
            Self::EMFILE => "Too many open files",
            Self::ENOTTY => "Not a typewriter",
            Self::ETXTBSY => "Text file busy",
            Self::EFBIG => "File too large",
            Self::ENOSPC => "No space left on device",
            Self::ESPIPE => "Illegal seek",
            Self::EROFS => "Read-only file system",
            Self::EMLINK => "Too many links",
            Self::EPIPE => "Broken pipe",
            Self::EDOM => "Math argument out of domain of func",
            Self::ERANGE => "Math result not representable",
            Self::EDEADLK => "Resource deadlock would occur",
            Self::ENAMETOOLONG => "File name too long",
            Self::ENOLCK => "No record locks available",
            Self::ENOSYS => "Function not implemented",
            Self::ENOTEMPTY => "Directory not empty",
            Self::ELOOP => "Too many symbolic links encountered",
            Self::EWOULDBLOCK => "Operation would block",
            Self::ENOMSG => "No message of desired type",
            Self::EIDRM => "Identifier removed",
            Self::ECHRNG => "Channel number out of range",
            Self::EL2NSYNC => "Level 2 not synchronized",
            Self::EL3HLT => "Level 3 halted",
            Self::EL3RST => "Level 3 reset",
            Self::ELNRNG => "Link number out of range",
            Self::EUNATCH => "Protocol driver not attached",
            Self::ENOCSI => "No CSI structure available",
            Self::EL2HLT => "Level 2 halted",
            Self::EBADE => "Invalid exchange",
            Self::EBADR => "Invalid request descriptor",
            Self::EXFULL => "Exchange full",
            Self::ENOANO => "No anode",
            Self::EBADRQC => "Invalid request code",
            Self::EBADSLT => "Invalid slot",
            Self::EDEADLOCK => "File locking deadlock error",
            Self::EBFONT => "Bad font file format",
            Self::ENOSTR => "Device not a stream",
            Self::ENODATA => "No data available",
            Self::ETIME => "Timer expired",
            Self::ENOSR => "Out of streams resources",
            Self::ENONET => "Machine is not on the network",
            Self::ENOPKG => "Package not installed",
            Self::EREMOTE => "Object is remote",
            Self::ENOLINK => "Link has been severed",
            Self::EADV => "Advertise error",
            Self::ESRMNT => "Srmount error",
            Self::ECOMM => "Communication error on send",
            Self::EPROTO => "Protocol error",
            Self::EMULTIHOP => "Multihop attempted",
            Self::EDOTDOT => "RFS specific error",
            Self::EBADMSG => "Not a data message",
            Self::EOVERFLOW => "Value too large for defined data type",
            Self::ENOTUNIQ => "Name not unique on network",
            Self::EBADFD => "File descriptor in bad state",
            Self::EREMCHG => "Remote address changed",
            Self::ELIBACC => "Can not access a needed shared library",
            Self::ELIBBAD => "Accessing a corrupted shared library",
            Self::ELIBSCN => ".lib section in a.out corrupted",
            Self::ELIBMAX => "Attempting to link in too many shared libraries",
            Self::ELIBEXEC => "Cannot exec a shared library directly",
            Self::EILSEQ => "Illegal byte sequence",
            Self::ERESTART => "Interrupted system call should be restarted",
            Self::ESTRPIPE => "Streams pipe error",
            Self::EUSERS => "Too many users",
            Self::ENOTSOCK => "Socket operation on non-socket",
            Self::EDESTADDRREQ => "Destination address required",
            Self::EMSGSIZE => "Message too long",
            Self::EPROTOTYPE => "Protocol wrong type for socket",
            Self::ENOPROTOOPT => "Protocol not available",
            Self::EPROTONOSUPPORT => "Protocol not supported",
            Self::ESOCKTNOSUPPORT => "Socket type not supported",
            Self::EOPNOTSUPP => "Operation not supported on transport endpoint",
            Self::EPFNOSUPPORT => "Protocol family not supported",
            Self::EAFNOSUPPORT => "Address family not supported by protocol",
            Self::EADDRINUSE => "Address already in use",
            Self::EADDRNOTAVAIL => "Cannot assign requested address",
            Self::ENETDOWN => "Network is down",
            Self::ENETUNREACH => "Network is unreachable",
            Self::ENETRESET => "Network dropped connection because of reset",
            Self::ECONNABORTED => "Software caused connection abort",
            Self::ECONNRESET => "Connection reset by peer",
            Self::ENOBUFS => "No buffer space available",
            Self::EISCONN => "Transport endpoint is already connected",
            Self::ENOTCONN => "Transport endpoint is not connected",
            Self::ESHUTDOWN => "Cannot send after transport endpoint shutdown",
            Self::ETOOMANYREFS => "Too many references: cannot splice",
            Self::ETIMEDOUT => "Connection timed out",
            Self::ECONNREFUSED => "Connection refused",
            Self::EHOSTDOWN => "Host is down",
            Self::EHOSTUNREACH => "No route to host",
            Self::EALREADY => "Operation already in progress",
            Self::EINPROGRESS => "Operation now in progress",
            Self::ESTALE => "Stale NFS file handle",
            Self::EUCLEAN => "Structure needs cleaning",
            Self::ENOTNAM => "Not a XENIX named type file",
            Self::ENAVAIL => "No XENIX semaphores available",
            Self::EISNAM => "Is a named type file",
            Self::EREMOTEIO => "Remote I/O error",
            Self::EDQUOT => "Quota exceeded",
            Self::ENOMEDIUM => "No medium found",
            Self::EMEDIUMTYPE => "Wrong medium type",
            Self::ECANCELED => "Operation Canceled",
            Self::ENOKEY => "Required key not available",
            Self::EKEYEXPIRED => "Key has expired",
            Self::EKEYREVOKED => "Key has been revoked",
            Self::EKEYREJECTED => "Key was rejected by service",
            Self::EOWNERDEAD => "Owner died",
            Self::ENOTRECOVERABLE => "State not recoverable",
            Self::ERFKILL => "Operation not possible due to RF-kill",
            Self::EHWPOISON => "Memory page has hardware error",
        }
    }
}

```

Once we have `Result` and `Error` we can reimplement our syscalls as follows:
```rust
use core::convert::TryInto;
use crate::error::{Error, Result};

#[no_mangle]
pub fn read(fd: i32, buf: &mut [u8]) -> Result<usize> {
    let rc = unsafe { syscall!(SYS_READ, fd, buf.as_ptr(), buf.len()) };

    if rc < 0 {
        return Err(Error::from(rc * -1))
    }

    Ok(rc.try_into().unwrap())
}


#[no_mangle]
pub fn write(fd: i32, buf: &[u8]) -> Result<usize> {
    let rc = unsafe { syscall!(SYS_WRITE, fd, buf.as_ptr(), buf.len()) };

    if rc < 0 {
        return Err(Error::from(rc * -1))
    }

    Ok(rc.try_into().unwrap())
}
```

and the `main` function should look like:
```rust
#[no_mangle]
fn main() -> u8 { 
    linux::syscall::write(1, b"Hello world\n").unwrap();
    0
}
```
Once we recompile and run the code we can see the text on the stdout
```
> ./cargo.sh run
Hello world
```
But what happens if we specify a wrong file number. Let's use 3 as file descriptor instead of 1. Since we never opened a file
with a descriptor 3 we should see an error now. Let's recompile and run
```
> ./cargo.sh run
```
our program starts hammering on the CPU and never exists. Sounds familiar? The `write` syscall returns and error, we unwrap it
and as a result our code panics, But we implemented our panic handler in the first chapter like this:
```rust
#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
```
Let's fix that calling the exit syscall instead of looping forever:
```rust
#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
    syscall::exit(255);
}
```
Now if we run the same code with file descriptor 3 the process should simply exit with error code 255.
```
> ./cargo.sh run; echo $?
255
```

## Standard IO
We already implemented `Display` and `Debug` for our error type so why don't we simply print them on the stderr?
The [`PanicInfo`](https://doc.rust-lang.org/core/panic/struct.PanicInfo.html) also implements these traits, so we should be
able to write them out, but how should we creata a string or more preciselly a bytearray from these types?
There is a nice macro in the core library called [`write!`](https://doc.rust-lang.org/core/macro.write.html) 
which could be used to format the output. Let's try that in the panic_handler function.
```rust
#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    write!(1u8, "{:?}", info);
    syscall::exit(255);
}
```
As you probably have expect, we get a compilation error:
```
./cargo.sh build
error[E0599]: cannot write into `u8`
  --> linux.rs:24:12
   |
24 |     write!(1u8, "{:?}", info);
   |     -------^^^--------------- method not found in `u8`
   |
note: must implement `io::Write`, `fmt::Write`, or have a `write_fmt` method
  --> linux.rs:24:12
   |
24 |     write!(1u8, "{:?}", info);
   |            ^^^
help: a writer is needed before this format string
  --> linux.rs:24:12
   |
24 |     write!(1u8, "{:?}", info);
   |            ^
```
We can not write into u8... Which kind of makes sense. The `write!` macro is part of the core library which has no 
idea about the `write` syscall we just implemented. We should somehow inverse the dependencies and the compiler message
helps us to do that. The first argument of the `write!` macro needs to implement the `io::Write`, `fmt::Write` traits 
or needs to have a `write_fmt` method. Let's wrap some integers into a struct and implement the 
[`fmt::Write`](https://doc.rust-lang.org/core/fmt/trait.Write.html) trait for it. 
(The [`io::Write`](https://doc.rust-lang.org/std/io/trait.Write.html) trait is part of the std library 
which we don't have access to)

Let's create a new module, called `io`. We need to include it into the `linux.rs` with `pub mode io;` and create a new
file called `io.rs` with the following content:
```rust
use core::fmt;
use crate::error::Result;

pub struct Stdio {
    fd: u32,
}

impl Stdio {
    pub fn read(&self, buf: &mut [u8]) -> Result<usize> {
        crate::syscall::read(self.fd, buf)
    }

    pub fn write(&self, buf: &[u8]) -> Result<usize> {
        crate::syscall::write(self.fd, buf)
    }
}

impl fmt::Write for Stdio {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        match self.write(s.as_bytes()) {
            Ok(_) => Ok(()),
            Err(_) => Err(fmt::Error),
        }
    }
}

pub fn stdin() -> Stdio {
    Stdio { fd: 0 }
}

pub fn stdout() -> Stdio {
    Stdio { fd: 1 }
}

pub fn stderr() -> Stdio {
    Stdio { fd: 2 }
}
```
After that we can rewrite the panic-handler like this:
```rust
use core::fmt::Write;

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    let _ = write!(io::stderr(), "{}\n", info);
    syscall::exit(255);
}
```
But if we try to build the code we get yet another linker error about the missing `memcpy` function. No problem. We already
expected that just didn't know when it is going to come. So let's put our `memcpy` implementation next to the `memset` in the
`ffi.rs` file:
```rust
#[no_mangle]
unsafe fn memcpy(dst: *mut u8, src: *const u8, len: usize) -> *mut u8 {
    for idx in 0 .. len {
        let offset = idx.try_into().unwrap();
        unsafe { 
            let byte = src.offset(offset).read(); 
            dst.offset(offset).write(byte); 
        }
    }
    dst
}
```
Exceptions in Rust: https://github.com/rust-lang/rfcs/blob/master/text/1236-stabilize-catch-panic.md
Last by not least we get an undefine reference error to `rust_eh_personality` TODO: what's this?
```rust
#![feature(lang_items)]
#![allow(internal_features)]

#[lang = "eh_personality"]
fn rust_eh_personality() {}
```

## Print macros
The write macro is already a big improvement but we can go further. Let's define two macros to print a text onto the
stdout and stderr. The can be defined in the io.rs file.
```rust
#[macro_export]
macro_rules! print {
    ($fmt:literal $(,$($args:expr)*)?) => {{
        use core::fmt::Write;
        write!($crate::io::stdout(), $fmt, $($($args),*)?).unwrap();
    }}
}

#[macro_export]
macro_rules! println {
    ($fmt:literal $(,$($args:expr)*)?) => {{
        $crate::print!("{}\n", format_args!($fmt, $($($args),*)?))
    }}
}

#[macro_export]
macro_rules! eprint {
    ($fmt:literal $(,$($args:expr)*)?) => {{
        use core::fmt::Write;
        write!($crate::io::stderr(), $fmt, $($($args),*)?).unwrap();
    }}
}

#[macro_export]
macro_rules! eprintln {
    ($fmt:literal $(,$($args:expr)*)?) => {{
        $crate::eprint!("{}\n", format_args!($fmt, $($($args),*)?))
    }}
}
```
and the `bin.rs` like this: (Note the new 
[`#[macro_use]`](https://doc.rust-lang.org/reference/macros-by-example.html#the-macro_use-attribute) attribute on the
extern linux crate)
```rust
#![no_std]
#![no_main]

#[macro_use]
extern crate linux;

#[no_mangle]
fn main() -> u8 { 
    print!("Hello");
    eprintln!(" {}", "world");
    0
}
```
