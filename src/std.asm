extern printf

section .data
buffer: db 0

section .rodata
tape_err: db `Error while initilizing tape: %llx\n\0`
input_err: db `Error while reading from stdin: %llx\n\0`
output_err: db `Error while writing to stdout: %llx\n\0`

section .text
output:
    ; read from tape to buffer
    mov al, byte [rbx]
    mov [buffer], al

    mov rax, 1        ; write
    mov rdi, 1        ; stdout
    lea rsi, [buffer] ; buf
    mov rdx, 1        ; len

    syscall
    cmp rax, 1
    jne .err
    
    ret

.err:
    lea rdi, [output_err] ; fmt
    mov rsi, rax        ; arg1
    call printf
    mov rdi, 1
    jmp exit

input:
    mov rax, 0        ; read
    mov rdi, 0        ; stdin
    lea rsi, [buffer] ; buf
    mov rdx, 1        ; len

    syscall
    cmp rax, 1
    jne .err

    ; read from buffer to tape
    mov al, [buffer]
    mov [rbx], al

    ret

.err:
    lea rdi, [input_err] ; fmt
    mov rsi, rax        ; arg1
    call printf
    mov rdi, 1
    jmp exit

init:
    mov rax, 9       ; mmap
    mov rdi, 0       ; addr
    mov rsi, 16*1024 ; len
    mov rdx, 0x3     ; prot: READ | WRITE
    mov r10, 0x22    ; flags: PRIVATE | ANONYMOUS
    mov r8, 0        ; fd
    mov r9, 0        ; off

    syscall
    cmp rax, 0
    jl .err

    mov rbx, rax

    ret

.err:
    lea rdi, [tape_err] ; fmt
    mov rsi, rax        ; arg1
    call printf
    mov rdi, 1
    jmp exit

exit:
    mov rax, 60
    syscall
    ud2

global main
main:
    call init
    call run
    mov rdi, 0
    call exit
