global exit

section .bss

exit:
  mov rax, 60
  syscall
