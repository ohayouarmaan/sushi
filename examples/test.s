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

 ; Loading immediate
 sub rsp, 8
 mov qword [rbp-24], 7

; Multiplying
 mov rax, [rbp-16]
 mov rcx, [rbp-24]
 imul rax, rcx
 sub rsp, 8
 mov [rbp-32], rax

 ; Loading immediate
 sub rsp, 8
 mov qword [rbp-40], 4

; Dividing
 mov rax, [rbp-32]
 mov rcx, [rbp-40]
 xor rdx, rdx
 div rcx
 sub rsp, 8
 mov [rbp-48], rax

; Adding
 mov rax, [rbp-8]
 mov rcx, [rbp-48]
 add rax, rcx
 sub rsp, 8
 mov qword [rbp-56], rax

 ; Storing variable

 ; Loading variable
 sub rsp, 8
 mov rax, [rbp-56]
 mov [rbp-64], rax

 ; Loading immediate
 sub rsp, 8
 mov qword [rbp-72], 3

 ; Loading immediate
 sub rsp, 8
 mov qword [rbp-80], 7

; Multiplying
 mov rax, [rbp-72]
 mov rcx, [rbp-80]
 imul rax, rcx
 sub rsp, 8
 mov [rbp-88], rax

 ; Loading immediate
 sub rsp, 8
 mov qword [rbp-96], 4

; Dividing
 mov rax, [rbp-88]
 mov rcx, [rbp-96]
 xor rdx, rdx
 div rcx
 sub rsp, 8
 mov [rbp-104], rax

; Adding
 mov rax, [rbp-64]
 mov rcx, [rbp-104]
 add rax, rcx
 sub rsp, 8
 mov qword [rbp-112], rax

 ; Storing variable

 ; Loading variable
 sub rsp, 8
 mov rax, [rbp-112]
 mov [rbp-120], rax

 ; Printing
 mov rdi, [rbp-120]
 call print_int

 xor rdi, rdi
 call exit

