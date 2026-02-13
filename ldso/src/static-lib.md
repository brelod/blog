# Static library

## Create static library

true.a
```
global true:function

section .data
RC: db 0x0

section .text
true:
    mov rdi,[RC]
    mov rax,0x3c
    syscall
```

false.s
```
global false:function

section .data
RC: db 0x1

section .text
false:
    mov rdi,[RC]
    mov rax,0x3c
    syscall
```

Compilation
```
> nasm -f elf64 true.o
> nasm -f elf64 false.o
> ar rcs bool.a true.o false.o
```

Display the content:
```
> ar tvO bool.a
rw-r--r-- 0/0    816 Jan  1 01:00 1970 true.o 0x96
rw-r--r-- 0/0    832 Jan  1 01:00 1970 false.o 0x402
```

Display symbols
```
> nm bool.a
true.o:
0000000000000000 d RC
0000000000000000 T true
false.o:
0000000000000000 T false
0000000000000000 d RC
```


## Link against static lib

```
> ld -static bin.o bool.a -o false
> ./false; echo $?
1
```

Display symbols
```
> nm false
0000000000402001 D __bss_start
0000000000402001 D _edata
0000000000402008 D _end
0000000000401010 T false
0000000000402000 d RC
0000000000401000 T _start
```
