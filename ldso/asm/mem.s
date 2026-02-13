default rel

global \
    memset:function,\
    memcpy:function,\
    memcmp:function,\
    strlen:function

; ==============================================================================
; Data
; ==============================================================================
section .data

; ==============================================================================
; text
; ==============================================================================
section .text

; void memset(void *s, int c, size_t n)
memset: 
    mov rax,rdi
.loop:
    dec rdx
    js .exit
    mov byte [rdi+rdx],sil
    jmp .loop
.exit:
    ret

; void memcpy(void *dst, void *src, size_t n)
memcpy:
    mov rax,rdi
.loop:
    dec rdx
    js .exit
    movsb
    jmp .loop
.exit:
    ret

; int memcmp(void *dst, void *src, size_t n)
memcmp:
.loop:
    dec rdx
    js .exit
    cmpsb
    jne .result
    jmp .loop
.result:
    mov rax,[rdi]
    sub rax,[rsi]
.exit:
    ret

; size_t strlen(const char *s)
strlen:
    xor rax,rax
.loop:
    cmp byte [rdi+rax],0x0
    je .exit
    inc rax
    jmp .loop
.exit:
    ret


