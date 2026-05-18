.intel_syntax noprefix

push rax
mov rax, QWORD PTR [rip+0x0]
mov rax, QWORD PTR [rax+0x1E508]
cmp rax, QWORD PTR [rbp+0x8]
pop rax
je 0x0
mov edx, DWORD PTR [r14+0x44]
lea rcx, [rsp+0x40]
jmp 0x0