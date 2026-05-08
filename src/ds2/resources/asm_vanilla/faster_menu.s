.intel_syntax noprefix

xor eax, ebp
mov DWORD PTR [ebp-0x4], eax
cmp BYTE PTR [ecx+0x4], 0x18
jne exit
mov BYTE PTR [ecx+0x4], 0x38
exit:
jmp 0x0