.intel_syntax noprefix

main:
cmp BYTE PTR [rip+0x0], 0x1
jne nothing_to_process
movabs r14, 0x0
mov r14, QWORD PTR [r14]
mov r13, r14
mov r14, QWORD PTR [r14+0xa8]
mov r14, QWORD PTR [r14+0x10]
mov r14, QWORD PTR [r14+0x10]
mov BYTE PTR [rip+0x0], 0x0
cmp BYTE PTR [rip+0x0], 0x1
jne skip_adjust
mov rcx, QWORD PTR [r14+0x10]
lea rdx, [rip+0x0]
lea r8, [rip+0x0]
mov r9d, DWORD PTR [rip+0x0]
movabs r15, 0x0
sub rsp, 0x30
call r15
add rsp, 0x30
movzx eax, WORD PTR [rip+0x0]
add eax, DWORD PTR [rip+0x0]
cmp eax, DWORD PTR [rip+0x0]
jle skip_adjust
mov eax, DWORD PTR [rip+0x0]
sub eax, DWORD PTR [rip+0x0]
mov WORD PTR [rip+0x0], ax
skip_adjust:
sub rsp, 0x208
mov rcx, r14
lea rdx, [rip+0x0]
mov r8d, DWORD PTR [rip+0x0]
xor r9d, r9d
movabs r15, 0x0
call r15
lea rcx, [rip+0x0]
lea rdx, [rip+0x0]
mov r8d, DWORD PTR [rip+0x0]
mov r9d, 0x1
movabs r15, 0x0
call r15
mov rcx, QWORD PTR [r13+0x22e0]
lea rdx, [rip+0x0]
movabs r15, 0x0
call r15
add rsp, 0x208
nothing_to_process:
movabs rax, 0x0
mov rcx, 0x5
call rax
cmp BYTE PTR [rip+0x0], 0x1
jne main
ret