global _start

section .text
_start:
    mov rdi,0x7
    mov rax,0x3c
    syscall
