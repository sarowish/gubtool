mov rax, QWORD PTR [rcx+0x8]
push rcx
mov rcx, QWORD PTR [rip+0x0]
cmp rcx, QWORD PTR [rax+0x8]
je skip
or DWORD PTR [rax+0x2C], 0x8
skip:
pop rcx
jmp 0x0