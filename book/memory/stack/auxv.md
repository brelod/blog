# Auxiliary vector

LD Magic:
```
> LD_DEBUG=bindings python
> LD_SHOW_AUXV=1 cat /dev/null
```
https://cseweb.ucsd.edu/~gbournou/CSE131/the_inside_story_on_shared_libraries_and_dynamic_loading.pdf

Let's check out the auxv passed by the kernel to the cat command:
```
> LD_SHOW_AUXV=1 cat /dev/null
AT_SYSINFO_EHDR:      0x7ffeeb1dd000
AT_MINSIGSTKSZ:       3632
AT_HWCAP:             f8bfbff
AT_PAGESZ:            4096
AT_CLKTCK:            100
AT_PHDR:              0x555a0920b040
AT_PHENT:             56
AT_PHNUM:             13
AT_BASE:              0x7f73bbebc000
AT_FLAGS:             0x0
AT_ENTRY:             0x555a0920e760
AT_UID:               1066129479
AT_EUID:              1066129479
AT_GID:               1065878017
AT_EGID:              1065878017
AT_SECURE:            0
AT_RANDOM:            0x7ffeeb0d43d9
AT_HWCAP2:            0x2
AT_EXECFN:            /usr/bin/cat
AT_PLATFORM:          x86_64
```
A bit lower level way to the the same is:
```
> od -t x8 /proc/self/auxv
0000000 0000000000000021 00007fff77dbd000
0000020 0000000000000033 0000000000000e30
0000040 0000000000000010 000000000f8bfbff
0000060 0000000000000006 0000000000001000
0000100 0000000000000011 0000000000000064
0000120 0000000000000003 00005633999f3040
0000140 0000000000000004 0000000000000038
0000160 0000000000000005 000000000000000d
0000200 0000000000000007 00007f43dc1f2000
0000220 0000000000000008 0000000000000000
0000240 0000000000000009 00005633999f6be0
0000260 000000000000000b 000000003f8bd847
0000300 000000000000000c 000000003f8bd847
0000320 000000000000000d 000000003f880201
0000340 000000000000000e 000000003f880201
0000360 0000000000000017 0000000000000000
0000400 0000000000000019 00007fff77d0daf9
0000420 000000000000001a 0000000000000002
0000440 000000000000001f 00007fff77d0dfec
0000460 000000000000000f 00007fff77d0db09
0000500 0000000000000000 0000000000000000
```

The parsing works similar to the ARGV / ENVP in env.rs:
```rust,no_run,no_playground
{{#include ../../../code/env.rs:auxv}}
```

And the startup logic looks like this now:
```rust,no_run,no_playground
pub(crate) static ARGV: AtomicPtr<*const i8> = AtomicPtr::new(core::ptr::null_mut());
pub(crate) static ENVP: AtomicPtr<*const i8> = AtomicPtr::new(core::ptr::null_mut());
pub(crate) static AUXV: AtomicPtr<auxv_t> = AtomicPtr::new(core::ptr::null_mut());

#[no_mangle]
unsafe fn __rust_main(rsp: *const u8) -> u8 {
    let argc = *(rsp as *const isize);
    let argv = rsp.offset(8) as *mut *const i8;
    let envp = rsp.offset(8 + 8 + argc * 8) as *mut *const i8;

    let mut p = envp;
    while !(*p).is_null() { 
        p = p.offset(1);
    }
    let auxv = p.offset(1) as *mut auxv_t;

    ARGV.store(argv, Ordering::Relaxed);
    ENVP.store(envp, Ordering::Relaxed);
    AUXV.store(auxv, Ordering::Relaxed);
}
```

