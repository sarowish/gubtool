.intel_syntax noprefix

mov QWORD PTR [rsp+0x150], rax
cmp DWORD PTR [rdi+0x8], 0x17
jne exit
mov DWORD PTR [rdi+0x8], 0x47
exit:
jmp 0x0