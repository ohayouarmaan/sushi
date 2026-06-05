global print_int
section .text

extern print_char

print_int:
    push rbx
    push r12
    mov rax, rdi
    mov rbx, 10
    xor r12, r12

.push_loop:
    xor rdx, rdx
    div rbx

    add dl, '0'

    push rdx

    inc r12

    cmp rax, 0
    jne .push_loop

.pop_loop:
    pop rdi
    call print_char

    dec r12
    cmp r12, 0
    jne .pop_loop

    pop r12
    pop rbx
    ret
