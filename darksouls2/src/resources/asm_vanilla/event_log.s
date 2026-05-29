push eax
push ebx
push edi
mov  edi, DWORD PTR ds:OFFSET write_index_1
mov  eax, edi
imul eax, eax, 0x5
lea  ebx, ds: OFFSET buffer
add  ebx, eax
mov  DWORD PTR [ebx], esi
mov  al, BYTE PTR [ebp+0xC]
mov  BYTE PTR [ebx+0x4], al
inc  edi
and  edi, 0x1FF
mov  DWORD PTR ds:OFFSET write_index_2, edi
pop  edi
pop  ebx
pop  eax
mov  eax, 0xD1B71759
jmp  set_event