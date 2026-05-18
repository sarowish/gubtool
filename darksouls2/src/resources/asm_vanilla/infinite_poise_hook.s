.intel_syntax noprefix

push eax
mov eax, ds:OFFSET game_man
cmp DWORD PTR [eax+0x74], ecx
jne normal
cmp DWORD PTR [ebx+0x5ec], 0xffffffff
jmp exit
normal:
cmp DWORD PTR [ebx+0x5ec], 0x0
exit:
pop eax
jmp OFFSET hook_loc