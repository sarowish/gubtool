mov rax, QWORD PTR [rcx+0x8]
push rcx
movabs rcx, OFFSET target_ptr_loc
mov rcx, QWORD PTR [rcx]
cmp rcx, QWORD PTR [rax+0x8]
je skip
or DWORD PTR [rax+0x2C], 0x8
skip:
pop rcx
jmp OFFSET hook_loc