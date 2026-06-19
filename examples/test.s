global _start
extern print_int
extern exit

section .text
_start:
	 push rbp
	 mov rbp, rsp
	 xor rdi, rdi
	 call exit

