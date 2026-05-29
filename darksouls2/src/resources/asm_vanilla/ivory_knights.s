cmp DWORD PTR [ecx+0x28], 0xA7F80
jne original
test eax, eax
jne original
cmp dl, 0xF
jne original
xor edx, edx
original:
mov BYTE PTR [eax+ecx*1+0x2A1], dl
jmp 0x0