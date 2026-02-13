default rel
global \
    env_init:function,\
    env_argv:function

extern \
    strlen:function

; ==============================================================================
; macro
; ==============================================================================
%define AT_NULL     0
%define AT_BASE     7
%define AT_ENTRY    9

; ==============================================================================
; bss
; ==============================================================================
section .bss
argv:       resq 0x1
envp:       resq 0x1
auxv:       resq 0x1
at_base:    resq 0x1
at_entry:   resq 0x1

; ==============================================================================
; text
; ==============================================================================
section .text

env_init:
.argv:
    lea rax,[rdi+0x8]
    mov [argv],rax
.envp:
    mov rax,[rdi]
    lea rdi,[rdi+0x8+rax*0x8+0x8] ; argc + argv + argc*8 + null
    mov [envp],rdi
    sub rdi,0x8
.auxv:
    add rdi,0x8 
    cmp qword [rdi],0x0
    jne .auxv
    add rdi,0x8
    mov [auxv],rdi
.parse_auxv:
    sub rdi,0x10
.loop:
    add rdi,0x10
    mov rax,[rdi+0x8]
    cmp qword [rdi],AT_BASE
    je .base
    cmp qword [rdi],AT_ENTRY
    je .entry
    cmp qword [rdi],AT_NULL
    jne .loop
    ret
.base:
    mov [at_base],rax
    jmp .loop
.entry:
    mov [at_entry],rax
    jmp .loop

env_argv:
    mov rax,[argv]
    ret

printa:
    push 0xa; new line + realign the stack
    mov r8,rdi
.loop:
    mov rdi,[r8]
    call strlen
    ; print text
    mov rdx,rax
    mov rsi,[r8]
    mov rdi,0x1
    mov rax,0x1
    syscall
    ; print new line
    mov rdx,0x1
    lea rsi,[rsp]
    mov rdi,0x1
    mov rax,0x1
    syscall
    ; loop pover
    add r8,0x8
    cmp qword [r8],0x0
    jne .loop
    pop rax
    ret

stack_print:
    mov rdi,[argv]
    call printa
    mov rdi,[envp]
    call printa
    ret
