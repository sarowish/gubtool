push rax
movabs rax, OFFSET game_man_imp
mov rax, QWORD PTR [rax]
cmp QWORD PTR [rax+0xD0], rcx
jne normal
cmp DWORD PTR [rbp+0x5EC], 0xFFFFFFFF
jmp exit
normal:
cmp DWORD PTR [rbp+0x5EC], ebx
exit:
pop rax
jmp OFFSET hook_loc