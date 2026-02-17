# File operations

- open, close
- stat, fstat, lstat, fstatat
- fcntl, fsync, fdatasync
- truncate, ftruncate, fallocate
- lseek
- seek, drop (close)
- BufRead, BufWrite -- prove with perf the many syscalls


## Open and close
First of all we need to implement two syscalls the `open` and the `close` to be able to work with files. If you lookup the
manual page of [`open`](https://man7.org/linux/man-pages/man2/open.2.html) and [`close`](https://man7.org/linux/man-pages/man2/close.2.html)
it says that the function signatures look like:
```c
int open(const char *path, int flags);
int close(int fd);
```
This should be quit simple to implement in Rust. Let's add the following functions to our `syscall.rs`:
```rust
const SYS_OPEN: isize = 2;
const SYS_CLOSE: isize = 3;

#[no_mangle]
pub fn open(path: &str, flags: u64, mode: u64) -> Result<u32> {
    let rc = unsafe { syscall!(SYS_OPEN, path.as_ptr(), flags, mode) };

    if rc < 0 {
        return Err(Error::from(rc * -1))
    } 

    Ok(u32::try_from(rc).unwrap())
}

#[no_mangle]
pub fn close(fd: u32) -> Result<()> {
    let rc = unsafe { syscall!(SYS_CLOSE, fd) };

    if rc < 0 {
        return Err(Error::from(rc * -1))
    } 

    Ok(())
}
```
And call them from the main function like this:
```rust
#[no_mangle]
fn main() -> u8 { 
    let fd = linux::syscall::open("./bin.rs", 0, 0).unwrap();
    linux::syscall::close(fd).unwrap();
    0
}
```
If try to run this code the following happens:
```
> ./cargo.sh run
panicked at ./bin.rs:10:50:
called `Result::unwrap()` on an `Err` value: ENAMETOOLONG
```
The error message is quite straighforward: The name of the file is too long. Heh? 8 character is too long? We have most
likely messed something up. So how does the kernel determine the length of our string? It uses the `strlen` function which
expects a string to be null terminated. As opposed to this the Rust [`str`](https://doc.rust-lang.org/core/primitive.str.html)
are not null terminated but it works as a byte [`slice`](https://doc.rust-lang.org/core/primitive.slice.html). As a result
the kernel does out of bound access on our str, so we just violated the rules of Rust, caused undefined behaviour and made
the whole library unsound. Nice...
We can prove it by adding a null byte into our str and letting the code run:
```rust
#[no_mangle]
fn main() -> u8 { 
    let fd = linux::syscall::open("./bin.rs\0", 0, 0).unwrap();
    linux::syscall::close(fd).unwrap();
    0
}
```
```
> ./cargo.sh run
```
Now seems to be all fine. But as the unsafe rules says: an unsafe block is only sound if it can not be called from safe code
in a way that it causes undefined behaviour. This means that we can not expect the user to put a null at the end of a str
every time a file needs to be opened. We have to convert the rust `str` into a null terminated string. And there is a nice struct
for it: [`CString`](https://doc.rust-lang.org/alloc/ffi/struct.CString.html). The only problem is that it is defined in the
`alloc` crate which we don't want to depend on. Let's avoid implementing our own allocation primitives for now and simply
use a stack array to build our null terminated string. So let's rewrite our `open` function like this:
```rust
#[no_mangle]
pub fn open(path: &str, flags: u64, mode: u64) -> Result<u32> {
    let mut dst = [0u8;crate::limits::PATH_MAX];
    let src = path.as_bytes();

    if src.len() >= crate::limits::PATH_MAX {
        return Err(Error::ENAMETOOLONG);
    }

    for idx in 0 .. src.len() {
        dst[idx] = src[idx];
    }

    let rc = unsafe { syscall!(SYS_OPEN, dst.as_ptr(), flags, mode) };

    if rc < 0 {
        return Err(Error::from(rc * -1))
    } 

    Ok(u32::try_from(rc).unwrap())
}
```
There are a couple of limits defined in the linux kernel. For example [here](https://github.com/torvalds/linux/blob/v6.9/include/uapi/linux/limits.h).
To conform to these limits, we include a module to the `linux.rs` with `pub mod limits;` and also create a file 
called `limits.rs` with the content of 
```rust
pub const PATH_MAX: usize = 4096;
```
After that we can remove the `\0` termination from our str and it should just work now:
```
> ./cargo.sh run
```
Let's define the options for the open syscall: You can the options in the [`fcntl.h`](https://github.com/torvalds/linux/blob/v6.9/include/uapi/asm-generic/fcntl.h)
And the opening mode flages in the [`stat.h`](https://github.com/torvalds/linux/blob/v6.9/include/uapi/linux/stat.h)
We can simply add these values into the `syscall.rs` file:
```rust
pub const O_ACCMODE:   u64 = 0o0000003;
pub const O_RDONLY:    u64 = 0o0000000;
pub const O_WRONLY:    u64 = 0o0000001;
pub const O_RDWR:      u64 = 0o0000002;
pub const O_CREAT:     u64 = 0o0000100;
pub const O_EXCL:      u64 = 0o0000200;
pub const O_NOCTTY:    u64 = 0o0000400;
pub const O_TRUNC:     u64 = 0o0001000;
pub const O_APPEND:    u64 = 0o0002000;
pub const O_NONBLOCK:  u64 = 0o0004000;
pub const O_DSYNC:     u64 = 0o0010000;
pub const O_DIRECT:    u64 = 0o0040000;
pub const O_LARGEFILE: u64 = 0o0100000;
pub const O_DIRECTORY: u64 = 0o0200000;
pub const O_NOFOLLOW:  u64 = 0o0400000;
pub const O_NOATIME:   u64 = 0o1000000;
pub const O_CLOEXEC:   u64 = 0o2000000;
pub const O_SYNC:      u64 = 0o4000000;
pub const O_PATH:      u64 = 0o10000000;
pub const O_TMPFILE:   u64 = 0o20000000;
pub const O_NDELAY:    u64 = O_NONBLOCK;

pub const S_IRWXU: u64 = 0o700; // RWX mask for owner
pub const S_IRUSR: u64 = 0o400; // R for ownwer
pub const S_IWUSR: u64 = 0o200; // W for ownwer
pub const S_IXUSR: u64 = 0o100; // X for ownwer

pub const S_IRWXG: u64 = 0o070; // RWX for group
pub const S_IRGRP: u64 = 0o040; // R for group
pub const S_IWGRP: u64 = 0o020; // W for group
pub const S_IXGRP: u64 = 0o010; // X for group

pub const S_IRWXO: u64 = 0o007; // RWX for other
pub const S_IROTH: u64 = 0o004; // R for other
pub const S_IWOTH: u64 = 0o002; // W for other
pub const S_IXOTH: u64 = 0o001; // X for other
```
So we can have a basic file handling functionality:
```rust
#[no_mangle]
fn main() -> u8 { 
    use linux::syscall::*;
    let fd = open("hello.txt", O_CREAT|O_RDWR|O_DSYNC, S_IRUSR|S_IWUSR).unwrap();
    write(fd, b"hello world\n").unwrap();
    close(fd).unwrap();
    0
}
```
And we can run it like this:
```
> ./cargo.sh run
> cat hello.txt
hello world
```

## stat, fstat, lstat
The C wrapper of the `stat` and `fstat` syscalls look like this:
```c
int stat(const char *pathname, struct stat *statbuf);
int fstat(int fd, struct stat *statbuf);
```
In C it's quite common to create a struct on the stack and pass it into a function as a pointer. The function initializes
the struct and after that we can use it. It makes a lot of sense because so we can use the return value as an error type.
Zero means typically that the function succeeded while something else means typically an error. As opposed to this we
have `Result` types in Rust. So would be better to create the `stat` struct on the stack of the syscall wrapper and
give it back as `Ok(stat)` in case of success? To find out let's implement two versions of this function:
```rust
const SYS_FSTAT: isize = 5;

#[repr(C)]
#[derive(Debug, Default)]
pub struct stat64 {
    pub st_dev: u64,
    pub st_ino: u64,
    pub st_nlink: u64,
    pub st_mode: u32,
    pub st_uid: u32,
    pub st_gid: u32,
    __pad0: i32,
    pub st_rdev: u64,
    pub st_size: i64,
    pub st_blksize: i64,
    pub st_blocks: i64,
    pub st_atime: i64,
    pub st_atime_nsec: i64,
    pub st_mtime: i64,
    pub st_mtime_nsec: i64,
    pub st_ctime: i64,
    pub st_ctime_nsec: i64,
    __reserved: [i64; 3],
}

#[no_mangle]
pub fn fstat1(fd: u32, stat: &mut stat64) -> Result<()> {
    let rc = unsafe { syscall!(SYS_FSTAT, fd, stat) };

    if rc < 0 {
        return Err(Error::from(rc * -1))
    } 

    Ok(())
}

#[no_mangle]
pub fn fstat2(fd: u32) -> Result<stat64> {
    let mut stat = stat64::default();
    let rc = unsafe { syscall!(SYS_FSTAT, fd, &mut stat) };

    if rc < 0 {
        return Err(Error::from(rc * -1))
    } 

    Ok(stat)
}
```
If we build the code and dump the assembly it's easy to see the difference between the two functions:
```
> ./cargo.sh build

> ./cargo.sh dump fstat1
0000000000401f70 <fstat1>:
  401f70:           55                          push   rbp
  401f71:           48 89 e5                    mov    rbp,rsp
  401f74:           b8 05 00 00 00              mov    eax,0x5
  401f79:           0f 05                       syscall
  401f7b:           48 85 c0                    test   rax,rax
  401f7e:       /-- 78 04                       js     401f84 <fstat1+0x14>
  401f80:       |   31 c0                       xor    eax,eax
  401f82:       |   5d                          pop    rbp
  401f83:       |   c3                          ret
  401f84:       \-> 48 f7 d8                    neg    rax
  401f87:           48 89 c7                    mov    rdi,rax
  401f8a:           5d                          pop    rbp
  401f8b:           ff 25 d7 4f 00 00           jmp    QWORD PTR [rip+0x4fd7]        # 406f68 <_GLOBAL_OFFSET_TABLE_+0x20>

> ./cargo.sh dump fstat2
0000000000401fa0 <fstat2>:
  401fa0:              55                       push   rbp
  401fa1:              48 89 e5                 mov    rbp,rsp
  401fa4:              53                       push   rbx
  401fa5:              48 81 ec 98 00 00 00     sub    rsp,0x98
  401fac:              89 f1                    mov    ecx,esi
  401fae:              48 89 fb                 mov    rbx,rdi
  401fb1:              0f 57 c0                 xorps  xmm0,xmm0
  401fb4:              0f 29 45 e0              movaps XMMWORD PTR [rbp-0x20],xmm0
  401fb8:              0f 29 45 d0              movaps XMMWORD PTR [rbp-0x30],xmm0
  401fbc:              0f 29 45 c0              movaps XMMWORD PTR [rbp-0x40],xmm0
  401fc0:              0f 29 45 b0              movaps XMMWORD PTR [rbp-0x50],xmm0
  401fc4:              0f 29 45 a0              movaps XMMWORD PTR [rbp-0x60],xmm0
  401fc8:              0f 29 45 90              movaps XMMWORD PTR [rbp-0x70],xmm0
  401fcc:              0f 29 45 80              movaps XMMWORD PTR [rbp-0x80],xmm0
  401fd0:              0f 29 85 70 ff ff ff     movaps XMMWORD PTR [rbp-0x90],xmm0
  401fd7:              0f 29 85 60 ff ff ff     movaps XMMWORD PTR [rbp-0xa0],xmm0
  401fde:              48 8d b5 60 ff ff ff     lea    rsi,[rbp-0xa0]
  401fe5:              b8 05 00 00 00           mov    eax,0x5
  401fea:              89 cf                    mov    edi,ecx
  401fec:              0f 05                    syscall
  401fee:              48 85 c0                 test   rax,rax
  401ff1:       /----- 78 13                    js     402006 <fstat2+0x66>
  401ff3:       |      48 8d 7b 08              lea    rdi,[rbx+0x8]
  401ff7:       |      ba 90 00 00 00           mov    edx,0x90
  401ffc:       |      ff 15 6e 4f 00 00        call   QWORD PTR [rip+0x4f6e]        # 406f70 <_GLOBAL_OFFSET_TABLE_+0x28>
  402002:       |      31 c0                    xor    eax,eax
  402004:       |  /-- eb 11                    jmp    402017 <fstat2+0x77>
  402006:       \--|-> 48 f7 d8                 neg    rax
  402009:          |   48 89 c7                 mov    rdi,rax
  40200c:          |   ff 15 56 4f 00 00        call   QWORD PTR [rip+0x4f56]        # 406f68 <_GLOBAL_OFFSET_TABLE_+0x20>
  402012:          |   88 43 01                 mov    BYTE PTR [rbx+0x1],al
  402015:          |   b0 01                    mov    al,0x1
  402017:          \-> 88 03                    mov    BYTE PTR [rbx],al
  402019:              48 89 d8                 mov    rax,rbx
  40201c:              48 81 c4 98 00 00 00     add    rsp,0x98
  402023:              5b                       pop    rbx
  402024:              5d                       pop    rbp
  402025:              c3                       ret
```

The second version of fstat is more thant twice as long as the first. But is it enough to throw it away? To be able to answer
the question we have to go a bit deeper in the code of `fstat2` and analyse what's actually happening here.

After aligning the satck (`push rbx`) we reserve `0x98` byte space on the stack for the `stat64` struct. This space has to be
zerod out and to make it fast the compiler zeros out the `xmm0` SIMD register and uses it to copy zeros on the stack.
```
  401fa0:              55                       push   rbp
  401fa1:              48 89 e5                 mov    rbp,rsp
  401fa4:              53                       push   rbx
  401fa5:              48 81 ec 98 00 00 00     sub    rsp,0x98
  401fac:              89 f1                    mov    ecx,esi
  401fae:              48 89 fb                 mov    rbx,rdi
  401fb1:              0f 57 c0                 xorps  xmm0,xmm0
  401fb4:              0f 29 45 e0              movaps XMMWORD PTR [rbp-0x20],xmm0
  401fb8:              0f 29 45 d0              movaps XMMWORD PTR [rbp-0x30],xmm0
  401fbc:              0f 29 45 c0              movaps XMMWORD PTR [rbp-0x40],xmm0
  401fc0:              0f 29 45 b0              movaps XMMWORD PTR [rbp-0x50],xmm0
  401fc4:              0f 29 45 a0              movaps XMMWORD PTR [rbp-0x60],xmm0
  401fc8:              0f 29 45 90              movaps XMMWORD PTR [rbp-0x70],xmm0
  401fcc:              0f 29 45 80              movaps XMMWORD PTR [rbp-0x80],xmm0
  401fd0:              0f 29 85 70 ff ff ff     movaps XMMWORD PTR [rbp-0x90],xmm0
  401fd7:              0f 29 85 60 ff ff ff     movaps XMMWORD PTR [rbp-0xa0],xmm0
```
Once we have initialized the struct we have to pass it together with the fd to the syscall
```
  401fde:              48 8d b5 60 ff ff ff     lea    rsi,[rbp-0xa0]
  401fe5:              b8 05 00 00 00           mov    eax,0x5
  401fea:              89 cf                    mov    edi,ecx
  401fec:              0f 05                    syscall
```
We check the return code of the syscall and if it's not zero we jump forward to the error handling (`401e63`)
```
  401fee:              48 85 c0                 test   rax,rax
  401ff1:       /----- 78 13                    js     402006 <fstat2+0x66>
```
If the return code was zero call `memcpy`. The paramters are `rdi` (dst) which is calculated from `rbx`, `rsi` (src) which
is the stat64 struct on the current function and `edx` (len) which is the size of the stat64 struct. So question is where
do we copy the initialized struct? If you look the first section of this code it says `mov rbx,rdi` which is kind of interesting
because `rdi` is used for the first parameter of the function calls which should be the filedescriptor in this case.
Let's investigate that in gdb (see bellow).
```
  401ff3:       |      48 8d 7b 08              lea    rdi,[rbx+0x8]
  401ff7:       |      ba 90 00 00 00           mov    edx,0x90
  401ffc:       |      ff 15 6e 4f 00 00        call   QWORD PTR [rip+0x4f6e]        # 406f70 <_GLOBAL_OFFSET_TABLE_+0x28>
  402002:       |      31 c0                    xor    eax,eax
  402004:       |  /-- eb 11                    jmp    402017 <fstat2+0x77>
```
Do the error handling here
```
  402006:       \--|-> 48 f7 d8                 neg    rax
  402009:          |   48 89 c7                 mov    rdi,rax
  40200c:          |   ff 15 56 4f 00 00        call   QWORD PTR [rip+0x4f56]        # 406f68 <_GLOBAL_OFFSET_TABLE_+0x20>
  402012:          |   88 43 01                 mov    BYTE PTR [rbx+0x1],al
  402015:          |   b0 01                    mov    al,0x1
```
Teardown the function and return with `Result<stat64>`. Release the `0x98` bytes and the extra 8 alignment byte from the stack
and return to the caller function.
```
  402017:          \-> 88 03                    mov    BYTE PTR [rbx],al
  402019:              48 89 d8                 mov    rax,rbx
  40201c:              48 81 c4 98 00 00 00     add    rsp,0x98
  402023:              5b                       pop    rbx
  402024:              5d                       pop    rbp
  402025:              c3                       ret
```

```
> gdb ./target/bin
(gdb) set disassembly-flavor intel
(gdb) break fstat2
(gdb) run
Breakpoint 1, linux::syscall::{impl#1}::default () at syscall.rs:52
52      #[derive(Debug, Default)]
(gdb) disassemble
Dump of assembler code for function linux::syscall::fstat2:
   0x0000000000401fa0 <+0>:     push   rbp
   0x0000000000401fa1 <+1>:     mov    rbp,rsp
   0x0000000000401fa4 <+4>:     push   rbx
   0x0000000000401fa5 <+5>:     sub    rsp,0x98
   0x0000000000401fac <+12>:    mov    ecx,esi
   0x0000000000401fae <+14>:    mov    rbx,rdi
=> 0x0000000000401fb1 <+17>:    xorps  xmm0,xmm0
...
(gdb) info registers esi rdi
esi            0x3                 3
rdi            0x7fffffffe7f8      140737488349176
```
Something is definitelly weird. The `esi` (alias `rsi`) which should contain the second parameter of the function is set to
the filedescriptor (3) and the `rdi` has some random address in it. But the `fstat2` doesn't even have two parameters...
So what's happening here? If we look up the 3.2.3 Parameter Passing chapter of the 
[System V ABI](https://refspecs.linuxbase.org/elf/x86_64-abi-0.99.pdf) and scroll down to the "Returning of Values" section
it has an interesting point:
> If the type has class MEMORY, then the caller provides space for the return
> value and passes the address of this storage in **rdi as** if it were the first
> argument to the function. In effect, this address becomes a **hidden first argument**. 
> This storage must not overlap any data visible to the callee through
> other names than this argument.
> On return %rax will contain the address that has been passed in by the
> caller in %rdi

So we could summarize the call to the two fstat functions as follows:
```rust
pub fn fstat1(fd: u32, stat: &mut stat64) -> Result<()>;
```
1. The caller reserves space for stat64
2. The caller zeros out stat64
3. fstat updates stat64
4. fstat returns the result

```rust
pub fn fstat2(fd: u32) -> Result<stat64>;
```
1. The caller reserves space for the first stat64
2. fstat reserves space for the second stat64
3. fstat zeros out the second stat64
4. fstat updates the second stat64
5. fstat overwrites the first stat64 with the second stat64
6. fstat returns the result

Beside the fact that the fstat1 function is much more lightweight (no extra allocation + memcpy) we can also reuse the
stat64 struct in case of checking multiple files. So we don't have to reintialize it over and over again, which was at
least 10 instruction long. As a conclusion let's drop the fstat2 function and rename fstat1 to fstat. 
Similarly we can also implement `stat` and `lstat` as follows
```rust
const SYS_STAT: isize = 4;
const SYS_FSTAT: isize = 5;
const SYS_LSTAT: isize = 6;

#[no_mangle]
pub fn stat(path: &str, stat: &mut stat64) -> Result<()> {
    let mut dst = [0u8;crate::limits::PATH_MAX];
    cpath(path.as_bytes(), &mut dst)?;

    let rc = unsafe { syscall!(SYS_STAT, dst.as_ptr(), stat) };

    if rc < 0 {
        return Err(Error::from(rc * -1))
    } 

    Ok(())
}

#[no_mangle]
pub fn fstat(fd: u32, stat: &mut stat64) -> Result<()> {
    let rc = unsafe { syscall!(SYS_FSTAT, fd, stat) };

    if rc < 0 {
        return Err(Error::from(rc * -1))
    } 

    Ok(())
}

#[no_mangle]
pub fn lstat(path: &str, stat: &mut stat64) -> Result<()> {
    let mut dst = [0u8;crate::limits::PATH_MAX];
    cpath(path.as_bytes(), &mut dst)?;

    let rc = unsafe { syscall!(SYS_LSTAT, dst.as_ptr(), stat) };

    if rc < 0 {
        return Err(Error::from(rc * -1))
    } 

    Ok(())
}
```
And now we can use them in the main function like this:
```rust
#[no_mangle]
fn main() -> u8 { 
    let fd = linux::syscall::open("hello", 0, 0).unwrap();
    let mut stat = linux::syscall::stat64::default();
    linux::syscall::fstat(fd, &mut stat).unwrap();
    println!("{:#?}", stat);
    0
}
```
The result should look something like this:
```
> ./cargo.sh run
stat64 {
    st_dev: 64768,
    st_ino: 940171,
    st_nlink: 1,
    st_mode: 33188,
    st_uid: 1066219479,
    st_gid: 1068570817,
    __pad0: 0,
    st_rdev: 0,
    st_size: 9,
    st_blksize: 4096,
    st_blocks: 8,
    st_atime: 1719926584,
    st_atime_nsec: 93534043,
    st_mtime: 1719926583,
    st_mtime_nsec: 457537512,
    st_ctime: 1719926583,
    st_ctime_nsec: 457537512,
    __reserved: [
        0,
        0,
        0,
    ],
}
```

## truncate, ftruncate, fallocate
To set the size of a file we can use the truncate and allocate syscall family. Let's implement these syscalls in `syscall.rs`:
```rust
const SYS_TRUNCATE: isize = 76;
const SYS_FTRUNCATE: isize = 77;
const SYS_FALLOCATE: isize = 285;

#[no_mangle]
pub fn truncate(path: &str, len: u64) -> Result<()> {
    let mut dst = [0u8;crate::limits::PATH_MAX];
    cpath(path.as_bytes(), &mut dst)?;

    let rc = unsafe { syscall!(SYS_TRUNCATE, dst.as_ptr(), len) };

    if rc < 0 {
        return Err(Error::from(rc * -1))
    } 

    Ok(())
}

#[no_mangle]
pub fn ftruncate(fd: u32, len: u64) -> Result<()> {
    let rc = unsafe { syscall!(SYS_FTRUNCATE, fd, len) };

    if rc < 0 {
        return Err(Error::from(rc * -1))
    } 

    Ok(())
}

#[no_mangle]
pub fn fallocate(fd: u32, mode: u32, offset: u64, len: u64) -> Result<()> {
    let rc = unsafe { syscall!(SYS_FALLOCATE, fd, mode, offset, len) };

    if rc < 0 {
        return Err(Error::from(rc * -1))
    } 

    Ok(())
}
```
We can use them like this:
```rust
#[no_mangle]
fn main() -> u8 { 
    use linux::syscall::*;
    let mut stat = stat64::default();
    let fd = open("buffer", O_CREAT|O_APPEND|O_RDWR, S_IRWXU).unwrap();

    fallocate(fd, 0, 0, 1024).unwrap();
    fstat(fd, &mut stat).unwrap();
    println!("size: {}", stat.st_size);

    ftruncate(fd, 512).unwrap();
    fstat(fd, &mut stat).unwrap();
    println!("size: {}", stat.st_size);

    close(fd).unwrap();
    0
}
```
So the result is:
```
> ./cargo.sh run
size: 1024
size: 512
```

## fsync, fdatasync
```rust
const SYS_FSYNC: isize = 74;
const SYS_FDATASYNC: isize = 75;

#[no_mangle]
pub fn fsync(fd: u32) -> Result<()> {
    let rc = unsafe { syscall!(SYS_FSYNC, fd) };

    if rc < 0 {
        return Err(Error::from(rc * -1))
    } 

    Ok(())
}

#[no_mangle]
pub fn fdatasync(fd: u32) -> Result<()> {
    let rc = unsafe { syscall!(SYS_FDATASYNC, fd) };

    if rc < 0 {
        return Err(Error::from(rc * -1))
    } 

    Ok(())
}
```

## lseek
The lseek sysall can be used to modify the cursor of the current file: The options can be found [here](https://github.com/torvalds/linux/blob/v6.9/include/uapi/linux/fs.h)
```rust
const SYS_LSEEK: isize = 8;

#[no_mangle]
pub fn lseek(fd: u32, offset: u64, whence: i32) -> Result<u64> {
    let rc = unsafe { syscall!(SYS_LSEEK, fd, offset, whence) };

    if rc < 0 {
        return Err(Error::from(rc * -1))
    } 

    Ok(u64::try_from(rc).unwrap())
}
```

The main function should look like this:
```rust
#[no_mangle]
fn main() -> u8 { 
    use linux::syscall::*;
    let fd = open("buffer", O_CREAT|O_APPEND|O_RDWR, S_IRWXU).unwrap();
    fallocate(fd, 0, 0, 1024).unwrap();

    let pos = lseek(fd, 512, SEEK_SET).unwrap();
    println!("Cursor position: {}", pos);
    read(0, &mut [0u8]).unwrap();

    close(fd).unwrap();
    0
}
```
Let's start our program and let it block on the read syscall
```
> ./cargo.sh run
Cursor position: 512
```
We can check the status of the file in the proc filesystem like this:
```
> cat /proc/$(pidof bin)/fdinfo/3
pos:    512
flags:  0102002
mnt_id: 30
ino:    940180
```

