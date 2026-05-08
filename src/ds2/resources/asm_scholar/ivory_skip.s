.intel_syntax noprefix

cmp edx, 0x20024678
jne check_next
cmp r8d, 0x1
jne exit
xor r8d, r8d
jmp exit
check_next:
cmp edx, 0x20024680
jne exit
cmp r8d, 0x1
jne exit
push rcx
push rdx
push r8
push r12
push r13
push r14
push r15
push rbx
push rbp
push rdi
push rsi
mov r12, 0x32250000
mov r13, 0x3
mov r14, 0x3009bde
mov rdi, 0x20024683
movabs r15, 0x0
movabs rbx, 0x0
movabs rbp, 0x0
mov rsi, rcx
sub rsp, 0x28
loop_start:
mov ecx, r12d
mov edx, r14d
call r15
lea rcx, [rax+0xb8]
mov rdx, rax
call rbx
mov rcx, [rax+0x48]
mov rax, [rcx]
mov edx, 0x46
call [rax+0x30]
mov rcx, rsi
mov edx, edi
mov r8d, 0x1
call rbp
inc r14
inc rdi
dec r13
jne loop_start
add rsp, 0x28
pop rsi
pop rdi
pop rbp
pop rbx
pop r15
pop r14
pop r13
pop r12
pop r8
pop rdx
pop rcx
exit:
mov [rsp+0x10], rsi
jmp 0x0