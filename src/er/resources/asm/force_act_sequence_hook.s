.intel_syntax noprefix

cmp BYTE PTR [rip+0x0], 0x1
jne normal
cmp dword ptr [rax+0x28], 0xfefefefe
jne normal
push rdx
push r8
mov edx, DWORD PTR [rip+0x0]
lea r8, [rip+0x0]
mov eax, DWORD PTR [r8+rdx*4]
inc edx
mov DWORD PTR [rip+0x0], edx
cmp edx, 0xB
jne skip_set_flag
mov BYTE PTR [rip+0x0], 0x0
skip_set_flag:
pop r8
pop rdx
jmp 0x0
normal:
movsx eax, BYTE PTR [rax+0xe9c1]
jmp 0x0