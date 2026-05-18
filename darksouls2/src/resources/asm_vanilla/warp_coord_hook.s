.intel_syntax noprefix

subps xmm0, xmm1
movaps XMMWORD PTR [esi+0x40], xmm0
push eax
mov eax, ds:0x0
mov eax, DWORD PTR [eax+0x280]
mov eax, DWORD PTR [eax+0xc]
mov eax, DWORD PTR [eax+0x168]
mov eax, DWORD PTR [eax+0xc]
mov eax, DWORD PTR [eax+0x4]
add eax, 0xe0
cmp eax, esi
jne exit
sub esp, 0x10
movups XMMWORD PTR [esp], xmm6
movups xmm6, XMMWORD PTR ds:0x0
movups XMMWORD PTR [esi+0x40], xmm6
movups xmm6, XMMWORD PTR ds:0x0
movups XMMWORD PTR [esi+0x50], xmm6
movups xmm6, XMMWORD PTR ds:0x0
movups XMMWORD PTR [esi+0x60], xmm6
movups xmm6, XMMWORD PTR ds:0x0
movups XMMWORD PTR [esi+0x70], xmm6
movups xmm6, XMMWORD PTR [esp]
add esp, 0x10
exit:
pop eax
jmp 0x0