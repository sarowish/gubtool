sub esp, 0x1FC
push esi
mov esi, OFFSET modify_once_flag
cmp DWORD PTR [esi], 0x0
jne skip
mov DWORD PTR [ecx+0x14], 0x7
mov DWORD PTR [esi], 0x1
skip:
pop esi
jmp hook_loc