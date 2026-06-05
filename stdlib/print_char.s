global print_char

section .bss
char_buf resb 1

section .text

print_char:
  
  mov [rel char_buf], dil

  mov rax, 1
  mov rdi, 1
  mov rsi, char_buf
  mov rdx, 1
  syscall

  ret
