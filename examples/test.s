global _start
extern print_int
extern exit

section .text
_start:
 push rbp
 mov rbp, rsp
 ; Loading immediate
 sub rsp, 8
 mov qword [rbp-8], 2

 ; Loading immediate
 sub rsp, 8
 mov qword [rbp-16], 3

; Adding
 mov rax, [rbp-8]
 mov rcx, [rbp-16]
 add rax, rcx
 sub rsp, 8
 mov qword [rbp-24], rax

 ; Storing variable

 ; Loading variable
 sub rsp, 8
 mov rax, [rbp-24]
 mov [rbp-32], rax

 ; Printing
 mov rdi, [rbp-32]
 call print_int

 xor rdi, rdi
 call exit

