sub rsp, 0x220
push rax
movabs rax, OFFSET modify_once_flag
cmp DWORD PTR [rax], 0x0
jne skip
mov DWORD PTR [rcx+0x28], 0x7
mov DWORD PTR [rax], 0x1
skip:
pop rax
jmp OFFSET hook_loc