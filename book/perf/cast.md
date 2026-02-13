# Casting

There are multiple ways to convert types in Rust. We're going to investigate the benefits and drawbacks of using one over another.
- [as](https://doc.rust-lang.org/reference/expressions/operator-expr.html#type-cast-expressions) keyword
- [Into](https://doc.rust-lang.org/core/convert/trait.Into.html) and
    [From](https://doc.rust-lang.org/core/convert/trait.From.html)
- [TryInto](https://doc.rust-lang.org/core/convert/trait.TryInto.html) and
    [TryFrom](https://doc.rust-lang.org/core/convert/trait.TryFrom.html)

## u64 => u32
Let's create a small program to check out the differences at the assembly level:
```rust
#![no_std]
#![no_main]

#[macro_use]
extern crate linux;

use core::convert::TryInto;
use core::convert::TryFrom;

#[no_mangle] #[inline(never)] fn cast_as(n: u32) -> u64 { n as u64 }
#[no_mangle] #[inline(never)] fn cast_into(n: u32) -> u64 { n.into() }
#[no_mangle] #[inline(never)] fn cast_from(n: u32) -> u64 { u64::from(n) }
#[no_mangle] #[inline(never)] fn cast_try_into(n: u32) -> u64 { n.try_into().unwrap() }
#[no_mangle] #[inline(never)] fn cast_try_from(n: u32) -> u64 { u64::try_from(n).unwrap() }

#[no_mangle]
fn main() -> u8 { 
    println!("{}", cast_as(1));
    println!("{}", cast_into(1));
    println!("{}", cast_from(1));
    println!("{}", cast_try_into(1));
    println!("{}", cast_try_from(1));
    0
}
```
once we compile, we get the following codes
```
> ./cargo.sh build
> ./cargo.sh dump cast_as
0000000000401270 <cast_as>:
  401270:       89 f8                   mov    eax,edi
  401272:       c3                      ret

> ./cargo.sh dump cast_into
0000000000401280 <cast_into>:
  401280:       89 f8                   mov    eax,edi
  401282:       c3                      ret

> ./cargo.sh dump cast_from
0000000000401290 <cast_from>:
  401290:       89 f8                   mov    eax,edi
  401292:       c3                      ret

> ./cargo.sh dump cast_try_into
00000000004012a0 <cast_try_into>:
  4012a0:       89 f8                   mov    eax,edi
  4012a2:       c3                      ret

> ./cargo.sh dump cast_try_from
00000000004012b0 <cast_try_from>:
  4012b0:       89 f8                   mov    eax,edi
  4012b2:       c3                      ret
```
As you can see rust really does a zero-cost abstraction and generates all of our functions the same way. This is possible
since a `u32` can always be converted into a `u64`. 

## u32 => u64
But What happens if we switch the types and try to convert `u64` into `u32`?
In this case we don't have the `Into` and `From` traits implemented for the conversion so we can only compare the following
functions:
```rust
#![no_std]
#![no_main]

use core::convert::TryInto;
use core::convert::TryFrom;

#[macro_use]
extern crate linux;

#[no_mangle] #[inline(never)] fn cast_as(n: u64) -> u32 { n as u32 }
#[no_mangle] #[inline(never)] fn cast_try_into(n: u64) -> u32 { n.try_into().unwrap() }
#[no_mangle] #[inline(never)] fn cast_try_from(n: u64) -> u32 { u32::try_from(n).unwrap() }

#[no_mangle]
fn main() -> u8 { 
    println!("{}", cast_as(1));
    println!("{}", cast_try_into(1));
    println!("{}", cast_try_from(1));
    0
}

```

Interestingly the code looks still the same. The compiler sees that the only value we use this function is a constant `1`
so it can be sure that it fits into an `u32` and it optimizes out the size checks.
```
> ./cargo.sh dump cast_as
0000000000401270 <cast_as>:
  401270:       89 f8                   mov    eax,edi
  401272:       c3                      ret

> ./cargo.sh dump cast_try_into
00000000004012a0 <cast_try_into>:
  4012a0:       89 f8                   mov    eax,edi
  4012a2:       c3                      ret

> ./cargo.sh dump cast_try_from
00000000004012b0 <cast_try_from>:
  4012b0:       89 f8                   mov    eax,edi
  4012b2:       c3                      ret
```

Let's make it a bit more comples by reading a random byte from stdin, so the compiler doesn't have a chance to optimize our code:
```rust
#![no_std]
#![no_main]

#[macro_use]
extern crate linux;

use core::convert::TryInto;
use core::convert::TryFrom;

#[no_mangle] #[inline(never)] fn cast_as(n: u64) -> u32 { n as u32 }
#[no_mangle] #[inline(never)] fn cast_try_into(n: u64) -> u32 { n.try_into().unwrap() }
#[no_mangle] #[inline(never)] fn cast_try_from(n: u64) -> u32 { u32::try_from(n).unwrap() }

#[no_mangle]
fn main() -> u8 { 
    let mut buf = [0u8;1];
    linux::syscall::read(0, &mut buf).unwrap();
    let n = buf[0].try_into().unwrap();

    println!("{}", cast_as(n));
    println!("{}", cast_try_into(n));
    println!("{}", cast_try_from(n));
    0
}
```

```
> ./cargo.sh dump cast_as
00000000004019c0 <cast_as>:
  4019c0:       48 89 f8                mov    rax,rdi
  4019c3:       c3                      ret

> ./cargo.sh dump cast_try_from
0000000000401a10 <cast_try_from>:
  401a10:           48 89 f8                    mov    rax,rdi
  401a13:           48 c1 e8 20                 shr    rax,0x20
  401a17:       /-- 75 03                       jne    401a1c <cast_try_from+0xc>
  401a19:       |   89 f8                       mov    eax,edi
  401a1b:       |   c3                          ret
  401a1c:       \-> 50                          push   rax
  401a1d:           48 8d 3d f2 37 00 00        lea    rdi,[rip+0x37f2]
  401a24:           48 8d 0d 7d 60 00 00        lea    rcx,[rip+0x607d]
  401a2b:           4c 8d 05 fe 60 00 00        lea    r8,[rip+0x60fe]
  401a32:           48 8d 54 24 07              lea    rdx,[rsp+0x7]
  401a37:           be 2b 00 00 00              mov    esi,0x2b
  401a3c:           ff 15 96 65 00 00           call   QWORD PTR [rip+0x6596]

> ./cargo.sh dump cast_try_into
00000000004019d0 <cast_try_into>:
  4019d0:           48 89 f8                    mov    rax,rdi
  4019d3:           48 c1 e8 20                 shr    rax,0x20
  4019d7:       /-- 75 03                       jne    4019dc <cast_try_into+0xc>
  4019d9:       |   89 f8                       mov    eax,edi
  4019db:       |   c3                          ret
  4019dc:       \-> 50                          push   rax
  4019dd:           48 8d 3d 32 38 00 00        lea    rdi,[rip+0x3832]
  4019e4:           48 8d 0d bd 60 00 00        lea    rcx,[rip+0x60bd]
  4019eb:           4c 8d 05 26 61 00 00        lea    r8,[rip+0x6126]
  4019f2:           48 8d 54 24 07              lea    rdx,[rsp+0x7]
  4019f7:           be 2b 00 00 00              mov    esi,0x2b
  4019fc:           ff 15 d6 65 00 00           call   QWORD PTR [rip+0x65d6]
```
Alright, that looks now a bit different. As we can see, the documentation of TryFrom and TryInto was right: the two functions
generate the same code, the only difference is how the rust code looks like. So from now on we don't care the try_from function either
and only compare the `as` keyword with the `TryInto` trait.

As you can see `as` keyword generates a single instruction in which it moves the content of `rdi` into `rax`. So it only does
register operation. As opposed to this the the `TryInto` trait generates 12 instructions. 5 of these instruction uses memory
access (lines with [...]) and although it points to code segment which is probably already located in L2 cache it's obviously
much slower than a simple register access. 

If we count one CPU cycle for moving the value of a register into another one and about 10 cycles for finding a value in L2 cache
we can say that the `TryInto` conversion takes about 60-70x longer. And if you do it a lot it quickly adds up and makes a huge
difference in the performance of your code. But is this really mirroring the reality? Well not quite...

If we look at the code a bit closer at the beginning it does the same as the `as` keyword. After that it shifts the value
of our number 32 bit right and if it's not equal to `0x4019dc` then it jumps to the failure handling logic.
```
  4019d0:           48 89 f8                    mov    rax,rdi
  4019d3:           48 c1 e8 20                 shr    rax,0x20
  4019d7:       /-- 75 03                       jne    4019dc <cast_try_into+0xc>
  4019d9:       |   89 f8                       mov    eax,edi
  4019db:       |   c3                          ret
```
This means that in case of the valid cast we only do 4 instructions which is only 4 times slower then the `as` keyword
but for that we get the benefit of the error handling. This makes the `.text` segment of our code obviously bigger and
so it will fit not so good into our instruction cache wich makes our code overall slower but it will be always correct.
As opposed to this we can see the `as` keyword a bit like an `unsafe` keyword which doesn't always produces the expected
value and it just goes forward like nothing had happened. Still it has the benefit of having smaller and faster code
if we use it with care.

To see the difference between the code sizes we can use readelf like this:
```
> readelf -s ./target/bin | grep -E 'cast_|Name'
   Num:    Value          Size Type    Bind   Vis      Ndx Name
    45: 00000000004019d0    50 FUNC    GLOBAL DEFAULT    2 cast_try_into
    95: 00000000004019c0     4 FUNC    GLOBAL DEFAULT    2 cast_as
```
This says that the cast_try_into function occupies 12.5x more space in our caches. And if we remove the function wrapper
from the casts (remove the `ret` instruction) casting with `as` takes 3 bytes while casting with `try_into` takes 49 bytes.
As a result we can have ~22 `as` cast and ~1.5 `try_into` cast in our L1 instruction cache. Which is quite a bit of difference.


Let's see what does the `as` keyword in different scenarios:

```rust
#![no_std]
#![no_main]

#[macro_use]
extern crate linux;

#[no_mangle] #[inline(never)] fn u32_as_u16(n: u32) -> u16 { n as u16 }
#[no_mangle] #[inline(never)] fn u32_as_i16(n: u32) -> i16 { n as i16 }
#[no_mangle] #[inline(never)] fn u32_as_i32(n: u32) -> i32 { n as i32 }
#[no_mangle] #[inline(never)] fn u32_as_u64(n: u32) -> u64 { n as u64 }
#[no_mangle] #[inline(never)] fn u32_as_i64(n: u32) -> i64 { n as i64 }

#[no_mangle] #[inline(never)] fn i32_as_u16(n: i32) -> u16 { n as u16 }
#[no_mangle] #[inline(never)] fn i32_as_i16(n: i32) -> i16 { n as i16 }
#[no_mangle] #[inline(never)] fn i32_as_u32(n: i32) -> u32 { n as u32 }
#[no_mangle] #[inline(never)] fn i32_as_u64(n: i32) -> u64 { n as u64 }
#[no_mangle] #[inline(never)] fn i32_as_i64(n: i32) -> i64 { n as i64 }


#[no_mangle]
fn main() -> u8 { 
    println!("u32_as_u16(1)         {}", u32_as_u16(1));
    println!("u32_as_i16(1)         {}", u32_as_i16(1));
    println!("u32_as_i32(1)         {}", u32_as_i32(1));
    println!("u32_as_u64(1)         {}", u32_as_u64(1));
    println!("u32_as_i64(1)         {}", u32_as_i64(1));

    println!("-----------------------------------------------");
    println!("u32_as_u16(u32::MAX)  {}", u32_as_u16(u32::MAX));
    println!("u32_as_i16(u32::MAX)  {}", u32_as_i16(u32::MAX));
    println!("u32_as_i32(u32::MAX)  {}", u32_as_i32(u32::MAX));
    println!("u32_as_u64(u32::MAX)  {}", u32_as_u64(u32::MAX));
    println!("u32_as_i64(u32::MAX)  {}", u32_as_i64(u32::MAX));

    println!("-----------------------------------------------");
    println!("i32_as_u16(i32::MIN)  {}", i32_as_u16(i32::MIN));
    println!("i32_as_i16(i32::MIN)  {}", i32_as_i16(i32::MIN));
    println!("i32_as_u32(i32::MIN)  {}", i32_as_u32(i32::MIN));
    println!("i32_as_u64(i32::MIN)  {}", i32_as_u64(i32::MIN));
    println!("i32_as_i64(i32::MIN)  {}", i32_as_i64(i32::MIN));

    println!("-----------------------------------------------");
    println!("i32_as_u16(-1)        {}", i32_as_u16(-1));
    println!("i32_as_i16(-1)        {}", i32_as_i16(-1));
    println!("i32_as_u32(-1)        {}", i32_as_u32(-1));
    println!("i32_as_u64(-1)        {}", i32_as_u64(-1));
    println!("i32_as_i64(-1)        {}", i32_as_i64(-1));

    println!("-----------------------------------------------");
    println!("i32_as_u16(1)         {}", i32_as_u16(1));
    println!("i32_as_i16(1)         {}", i32_as_i16(1));
    println!("i32_as_u32(1)         {}", i32_as_u32(1));
    println!("i32_as_u64(1)         {}", i32_as_u64(1));
    println!("i32_as_i64(1)         {}", i32_as_i64(1));

    println!("-----------------------------------------------");
    println!("i32_as_u16(i32::MAX)  {}", i32_as_u16(i32::MAX));
    println!("i32_as_i16(i32::MAX)  {}", i32_as_i16(i32::MAX));
    println!("i32_as_u32(i32::MAX)  {}", i32_as_u32(i32::MAX));
    println!("i32_as_u64(i32::MAX)  {}", i32_as_u64(i32::MAX));
    println!("i32_as_i64(i32::MAX)  {}", i32_as_i64(i32::MAX));

    0
}
```
Interestingly if we try to disasseble the code of these functions there are only three of them can be found:
`i32_as_i16`, `u32_as_i64`, `i32_as_i64`. We must to dig a bit deeper to find out why. Let's checkout the symbol table
of the binary with 
```
> objdump -t ./target/bin | grep _as_ | sort
0000000000401270 g     F .text  0000000000000003 u32_as_i64
0000000000401270 g     F .text  0000000000000003 u32_as_u64
0000000000401280 g     F .text  0000000000000003 i32_as_i16
0000000000401280 g     F .text  0000000000000003 i32_as_u16
0000000000401280 g     F .text  0000000000000003 u32_as_i16
0000000000401280 g     F .text  0000000000000003 u32_as_u16
0000000000401290 g     F .text  0000000000000004 i32_as_i64
0000000000401290 g     F .text  0000000000000004 i32_as_u64
```
In this output we can see the following columns:
- memory address
- flags to describe the type of the symbol (g=global, F=function)
- section in which the symbol is located (.text = program code)
- size of the symbol
- name of the symbol

If you have a closer look at the memory address of the symboles you can see that multiple symbol uses the same address.
This means that there are only 3 different code sections for these 8 symboles. In the disassemble function of the objdump
command it takes only the first of these memory addresses as real symbol and so it doesn't find any other of them.
The reason for merging these symboles are that they do exactly the same from the compilers perspective. Let's see what
is that:
```
> objdump --disassemble=u32_as_i64 -M intel ./target/bin
0000000000401270 <u32_as_i64>:
  401270:       89 f8                   mov    eax,edi
  401272:       c3                      ret

> objdump --disassemble=i32_as_i16 -M intel ./target/bin
0000000000401280 <i32_as_i16>:
  401280:       89 f8                   mov    eax,edi
  401282:       c3                      ret

> objdump --disassemble=i32_as_i64 -M intel ./target/bin
0000000000401290 <i32_as_i64>:
  401290:       48 63 c7                movsxd rax,edi
  401293:       c3                      ret
```
I'm now sure why the first two functions weren't merged but maybe because of the different input type (`u32`/`i32`) but the
third function is obviously different. It creates a signed integer with bigger size. This means that the value has to be
sign-extended ([`movsxd`](https://www.felixcloutier.com/x86/movsx:movsxd). This means for example if case of `i8 => i16`
```
-1: 0xff => 0xffff
+1: 0x01 => 0x0001
```

Last but not least, let's see the output of our program. We have the following blocks:

Unsigned normal:
```
u32_as_u16(1)         1                     # same
u32_as_i16(1)         1                     # same
u32_as_i32(1)         1                     # same
u32_as_u64(1)         1                     # same
u32_as_i64(1)         1                     # same
```

Unsigned overflow:
```
u32_as_u16(u32::MAX)  65535                 # diff (truncated)
u32_as_i16(u32::MAX)  -1                    # diff (2's complement)
u32_as_i32(u32::MAX)  -1                    # diff (2's complement)
u32_as_u64(u32::MAX)  4294967295            # same
u32_as_i64(u32::MAX)  4294967295            # same
```

Signed underflow:
```
i32_as_u16(i32::MIN)  0                     # diff
i32_as_i16(i32::MIN)  0                     # diff
i32_as_u32(i32::MIN)  2147483648            # diff (not 2's complement)
i32_as_u64(i32::MIN)  18446744071562067968  # diff (not 2's complement)
i32_as_i64(i32::MIN)  -2147483648           # same

i32_as_u16(-1)        65535                 # diff (not 2's complement)
i32_as_i16(-1)        -1                    # same
i32_as_u32(-1)        4294967295            # diff (not 2's complement)
i32_as_u64(-1)        18446744073709551615  # diff (not 2's complement)
i32_as_i64(-1)        -1                    # same
```
Signed normal:
```
i32_as_u16(1)         1                     # same
i32_as_i16(1)         1                     # same
i32_as_u32(1)         1                     # same
i32_as_u64(1)         1                     # same
i32_as_i64(1)         1                     # same
```

Signed overflow:
```
i32_as_u16(i32::MAX)  65535                 # diff (truncated)
i32_as_i16(i32::MAX)  -1                    # diff (2's complement)
i32_as_u32(i32::MAX)  2147483647            # same
i32_as_u64(i32::MAX)  2147483647            # same
i32_as_i64(i32::MAX)  2147483647            # same
```

As a result we can set up the following rules: 

| Cast       | Safe if                          |
|------------|----------------------------------|
| `uS as uB` | always                           |
| `uS as iB` | always                           |
| `uN as iN` | `uN <= iN::MAX`                  |
| `iN as uN` | `iN >= uN::MIN`                  |
| `uB as uS` | `uB <= uS::MAX`                  |
| `iB as uS` | `iB >= iS::MIN && iB <= iS::MAX` |
| `iB as uS` | `iB >= uS::MIN && iB <= uS::MAX` |
| `uB as uS` | `uB <= iS::MAX`                  |

Where the letters have the following meanings:
- S: small
- B: big
- N: number (same size)
- u: unsigned
- i: signed

As a conclusion we could say the followings about casting integers:
- Use the `From`, `Into` traits whenever the type system allows it. It's the safest way and it doesn't have any overhead.
- Use the `TryFrom`, `TryInto` traits whenever the type system requires it and you can not be sure about the input value.
Even though it has a bit of an overhead and it decreases the cache locality of your code but it's always safe to use and
let's the compiler warn you if you unintetionally modify the code in an incorrect way later on.
- Use the `as` keyword instead of `TryFrom` and `TryInto` only if you can always be sure about the input number. Even though
it doesn't require an `unsafe` block it's easy to shoot you into the foot by refactoring the code without
realizing that the input value is not deterministic anymore. In this case you will have hard to determined bugs.
