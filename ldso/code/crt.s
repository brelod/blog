global _start
extern main:function

section .text
_start:
    xor rbp,rbp
    and rsp,-16
    mov rdi,rsp
    call main
    mov rdi,rax
    mov rax,0x3c
    syscall
