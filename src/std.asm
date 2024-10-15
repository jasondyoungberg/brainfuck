extern printf
extern scanf
extern exit
extern getchar
extern putchar
extern malloc

section .text
global main
main:
    and rsp, -16 ; algin stack

    mov rdi, 16 * 1024 ; tape size
    call malloc
    test rax, rax
    jz error
    mov rbx, rax ; init tape

    call run

    xor rdi, rdi
    call exit
    ud2

output EQU output_std
input EQU input_std

section .text
output_std:
    movzx rdi, byte [rbx] ; read from tape
    call putchar
    test rax, rax
    js error
    ret

section .text
output_num:
    lea rdi, [.fmt]
    movzx rsi, byte [rbx] ; read from tape
    call printf
    test rax, rax
    js error
    ret
section .rodata
    .fmt: db `output: %d\n`, 0

section .text
input_std:
    call getchar
    test rax, rax
    js error
    mov [rbx], al ; write to tape
    ret

section .text
input_num:
    lea rdi, [.print_fmt]
    call printf
    test rax, rax
    js error

    lea rdi, [.scan_fmt]
    lea rsi, [.buffer]
    call scanf
    test rax, rax
    js error

    mov al, [.buffer]
    mov [rbx], al ; write to tape
    ret
section .data
    .buffer: dd 0
section .rodata
    .print_fmt: db `input: `, 0
    .scan_fmt: db `%d`, 0

section .text
error:
    mov rdi, 1
    call exit
    ud2
