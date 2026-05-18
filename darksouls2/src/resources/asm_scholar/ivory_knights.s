.intel_syntax noprefix

cmp DWORD PTR [rcx+0x4c], 0xa7f80
jne normal
test rax, rax
jne normal
cmp r8d, 0xf
jne normal
xor r8d, r8d
normal:
mov BYTE PTR [rax+rcx*1+0x3a1], r8b
jmp 0x0