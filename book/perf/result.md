# Result

Let's start with a very simple code:
```rust
#![no_std]
#![no_main]

#[macro_use]
extern crate linux;

use core::arch::asm;

#[no_mangle]
#[inline(never)] 
fn ok() -> Result<(), ()> { 
    unsafe { asm!( "nop", options(nostack)) };
    Ok(()) 
}
#[no_mangle]
#[inline(never)]
fn err() -> Result<(), ()> { 
    unsafe { asm!( "nop", options(nostack)) };
    Err(())
}

#[no_mangle]
fn main() -> u8 { 
    println!("{:?}", ok());
    println!("{:?}", err());
    0
}

```
Rust does a good job with optimizing out the code which is not necessary and this feautre makes it difficult to 
investigate the code of a function. A simple function with a constant return value wont be put into the binary so we can not
dump the assembly of it. To trick the compiler into leaving our code in the binary we can use a simple assembly line which
doesn't do anything. The compiler doesn't check the value of it, it just thinks that it's important so it
leaves it there. After the compilation our code looks like this:
```
> ./cargo.sh dump ok
00000000004012f0 <ok>:
  4012f0:       90                      nop
  4012f1:       31 c0                   xor    eax,eax
  4012f3:       c3                      ret

> ./cargo.sh dump err
0000000000401300 <err>:
  401300:       90                      nop
  401301:       b0 01                   mov    al,0x1
  401303:       c3                      ret
```
As you can see the code of the two functions are quite similar. The `rax` register will be set and the code
returns. In case of `Ok(())` the `rax` is set to zero and by `Err(())` it will be set to one. Since the content of the
result is always zero sized `()` it doesn't even need a register to be passed back to the caller function.

Let's modify our code to return with some real value: for example an `u32` number:
```rust
#![no_std]
#![no_main]

#[macro_use]
extern crate linux;

use core::arch::asm;

#[no_mangle]
#[inline(never)] 
fn ok() -> Result<u32, ()> { 
    unsafe { asm!( "nop", options(nostack)) };
    Ok(3) 
}
#[no_mangle]
#[inline(never)]
fn err() -> Result<(), u32> { 
    unsafe { asm!( "nop", options(nostack)) };
    Err(3)
}

#[no_mangle]
fn main() -> u8 { 
    println!("{:?}", ok());
    println!("{:?}", err());
    0
}
```
This changes already the output of the dump
```
> ./cargo.sh dump ok
0000000000401360 <ok>:
  401360:       90                      nop
  401361:       31 c0                   xor    eax,eax
  401363:       ba 03 00 00 00          mov    edx,0x3
  401368:       c3                      ret

> ./cargo.sh dump err
0000000000401370 <err>:
  401370:       90                      nop
  401371:       b8 01 00 00 00          mov    eax,0x1
  401376:       ba 03 00 00 00          mov    edx,0x3
  40137b:       c3                      ret
```
As you can see the value of `Result` is passed back to the caller in another register `rdx`. This alligns with the
[System V ABI](https://refspecs.linuxbase.org/elf/x86_64-abi-0.99.pdf).

But what happens if we use some realistic error type, like an Error enum?
```rust
#![no_std]
#![no_main]

#[macro_use]
extern crate linux;

use core::arch::asm;

#[derive(Debug, Clone)]
pub enum Error { A }

#[no_mangle]
#[inline(never)]
fn ok() -> Result<(), Error> { 
    unsafe { asm!( "nop", options(nostack)) };
    Ok(())
}

#[no_mangle]
#[inline(never)]
fn err() -> Result<(), Error> { 
    unsafe { asm!( "nop", options(nostack)) };
    Err(Error::A)
}

#[no_mangle]
fn main() -> u8 { 
    println!("{:?}", ok());
    println!("{:?}", err());
    0
}

```
It looks quit similar, right?
```
> ./cargo.sh dump ok
0000000000401310 <ok>:
  401310:       90                      nop
  401311:       31 c0                   xor    eax,eax
  401313:       c3                      ret

> ./cargo.sh dump err
0000000000401320 <err>:
  401320:       90                      nop
  401321:       b0 01                   mov    al,0x1
  401323:       c3                      ret
```

 Then add another error variant to the Error enum and try it again
```rust
#[derive(Debug, Clone)]
pub enum Error { A, B }
```
```
> ./cargo.sh dump ok
0000000000401320 <ok>:
  401320:       90                      nop
  401321:       b0 02                   mov    al,0x2
  401323:       c3                      ret

> ./cargo.sh dump err
0000000000401330 <err>:
  401330:       90                      nop
  401331:       31 c0                   xor    eax,eax
  401333:       c3                      ret
```
The return values have been changed. Now zero means `Err(Error::A)` and two means `Ok(())`. It seems like the compiler
realizes that the `Ok(())` value can only have one state so it can be represented just like another variant of the `Error`
enum. It kind of creates another enum under the hood like
```rust
enum SpecialError {
    Err_Error_A = 0,
    Err_Error_B = 1,
    Ok = 2,
}
```
This way it's enough to use only one register instead of two. Pretty nice, right?
Let's return with some real `Ok()` value to avoid this optimisation. For example a number like this:
```rust
#[no_mangle]
#[inline(never)]
fn ok() -> Result<u8, Error> { 
    unsafe { asm!( "nop", options(nostack)) };
    Ok(3)
}

#[no_mangle]
#[inline(never)]
fn err() -> Result<u8, Error> { 
    unsafe { asm!( "nop", options(nostack)) };
    Err(Error::B)
}
```
With u8 it seems to be quite good. `rax=0` means `Ok` `rax=1` means `Err` and the `rdx` holds the value.
```
> ./cargo.sh dump ok
0000000000401320 <ok>:
  401320:       90                      nop
  401321:       31 c0                   xor    eax,eax
  401323:       b2 03                   mov    dl,0x3
  401325:       c3                      ret

> ./cargo.sh dump err
0000000000401330 <err>:
  401330:       90                      nop
  401331:       b0 01                   mov    al,0x1
  401333:       b2 01                   mov    dl,0x1
  401335:       c3                      ret
```

But what happens if we try to return with a bigger number like `i32`?
```rust
#[no_mangle]
#[inline(never)]
fn ok() -> Result<i32, Error> { 
    unsafe { asm!( "nop", options(nostack)) };
    Ok(3)
}

#[no_mangle]
#[inline(never)]
fn err() -> Result<i32, Error> { 
    unsafe { asm!( "nop", options(nostack)) };
    Err(Error::B)
}
```
It get's already a bit scarry
```
> ./cargo.sh dump ok
0000000000401330 <ok>:
  401330:       90                      nop
  401331:       48 b8 00 00 00 00 03    movabs rax,0x300000000
  401338:       00 00 00
  40133b:       c3                      ret

> ./cargo.sh dump err
0000000000401340 <err>:
  401340:       90                      nop
  401341:       b8 01 01 00 00          mov    eax,0x101
  401346:       c3                      ret
```
I assume since the Result is an enum too which size must be equal to the tag size + the size of the biggest inner value
it tries to encode the tag and the `i32` inner value into a 64 bit register. The first half of the register represents the
tag (zero = Ok) and the second half the `i32` value (3). As opposed to this the will still be encoded as a single integer
but since the value of the `Error` enum can not be bigger then 2 the compiler doesn't have to use a full 64 bit register.

And it's just the beginning. Replace `i32` with `i64` and you'll get this:
```
> ./cargo.sh dump ok
0000000000401330 <ok>:
  401330:       48 89 f8                mov    rax,rdi
  401333:       90                      nop
  401334:       48 c7 47 08 03 00 00    mov    QWORD PTR [rdi+0x8],0x3
  40133b:       00
  40133c:       c6 07 00                mov    BYTE PTR [rdi],0x0
  40133f:       c3                      ret

> ./cargo.sh dump err
0000000000401340 <err>:
  401340:       48 89 f8                mov    rax,rdi
  401343:       90                      nop
  401344:       66 c7 07 01 01          mov    WORD PTR [rdi],0x101
  401349:       c3                      ret
```
This is even more hairy... The System V ABI says that if you need to return two integer values you can use the `rax` and `rdx`
registers just like we did this above. As opposed to this if the return value has a MEMORY type (eg a big struct) then
the caller functions needs to reserve space for the return value and the called function will write the value there.
In this case the pointer to this space is passed as a hidden first argument to the function in the `rdi` register and the
`rax` register should hold the pointer to this space at the return point. Hence `mov rax,rdi` at the beginning of both functions.
And after that the provided memory location pointed by `[rdi]` will be filled with either `0` and `0x3` for the `Ok(3)` or
with `0x101` for the `Err(Error::B)`

And this is kind of sad because we're most likely hitting L2 (but at a minimum L1) cache for returning a 
simple number as number instead of passing it back in two registers basically for free.

It can be corrected by forcing the compiler to use the C ABI with the `extern "C"` declaration:
```rust
#[no_mangle]
#[inline(never)]
extern "C" fn ok() -> Result<usize, Error> { 
    unsafe { asm!( "nop", options(nostack)) };
    Ok(3)
}

#[no_mangle]
#[inline(never)]
extern "C" fn err() -> Result<usize, Error> { 
    unsafe { asm!( "nop", options(nostack)) };
    Err(Error::B)
}
```
We'll see a warning that the `Result` enum is not FFI-safe but we also not want to use it currently as an FFI function.
Just as a function which doesn't do unnecessarry work:
```
> ./cargo.sh dump ok
0000000000401330 <ok>:
  401330:       90                      nop
  401331:       ba 03 00 00 00          mov    edx,0x3
  401336:       31 c0                   xor    eax,eax
  401338:       c3                      ret

> ./cargo.sh dump err
0000000000401340 <err>:
  401340:       90                      nop
  401341:       b8 01 01 00 00          mov    eax,0x101
  401346:       c3                      ret
```

So as long as we don't use an `Ok` or `Err` type bigger than 64 bit we should be good now.
Even though it's a bit compilicated to use there are some benefits of using `Result`:
- The value must be checked instead of simply using (like by malloc). You can not access
    its content until you proved that it's has an Ok or Err value. This is a huge benefit.
- The questionmark. You can forward the error by using simply `Err()?`. But how does it work
    under the hood?

