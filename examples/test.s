global _start
extern print_int
extern exit
section .text
_start:
 push rbp
 mov rbp, rsp
 sub rsp, 8
 mov qword [rbp-8], 123
 sub rsp, 8
 mov qword [rbp-16], 234
 mov rax, [rbp-8]
 mov rcx, [rbp-16]
 add rax, rcx
 mov qword [rbp-24], rax
 sub rsp, 8
 mov qword [rbp-32], 999
 mov rax, [rbp-24]
 mov rcx, [rbp-32]
 add rax, rcx
 mov qword [rbp-40], rax
 mov rdi, [rbp-40]
 call print_int
 mov rdi, 4
 call exit
