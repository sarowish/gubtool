push rcx
movabs rcx, OFFSET game_man_imp
mov rcx, QWORD PTR [rcx]
mov rcx, QWORD PTR [rcx+0xd0]
cmp rcx, rbx
jne exit
mov eax, DWORD PTR [rcx+0x168]
exit:
pop rcx
mov DWORD PTR [rbx+0x168], eax
jmp OFFSET hook_loc