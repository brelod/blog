%include "const.mac"

default rel
global _start
extern \
    strlen:function,\
    env_init:function,\
    env_argv:function


section .text

_start:
    mov rdi,rsp
    call env_init

    call env_argv
    mov rdi,[rax]
    mov rax,SYS_OPEN
    mov rsi,O_RDONLY
    syscall

    mov rax,SYS_PAUSE
    syscall

    mov rdi,0
    mov rax,0x3c
    syscall
