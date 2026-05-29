cmp DWORD PTR [esp+0x4], 0x20024680
jne original_set_event
cmp DWORD PTR [esp+0x8], 0x1
jne original_set_event
push ebp
mov ebp, esp
sub esp, 0x18
push edi
push ecx
push edx
push ebx
mov edi, ecx
mov ebx, 0x0
mov DWORD PTR [ebp-0x4], 0x32250000
mov DWORD PTR [ebp-0x8], 0x3
mov DWORD PTR [ebp-0xC], 0x3009BDE
mov DWORD PTR [ebp-0x10], 0x20024683
mov DWORD PTR [ebp-0x14], 0x0
mov DWORD PTR [ebp-0x18], 0x0
loop_start:
push DWORD PTR [ebp-0xC]
push DWORD PTR [ebp-0x4]
call DWORD PTR [ebp-0x14]
add esp, 0x8
mov ecx, eax
push ecx
add ecx, 0x84
call DWORD PTR [ebp-0x18]
mov ecx, eax
add ecx, 0x24
mov ecx, DWORD PTR [ecx]
mov edx, DWORD PTR [ecx]
mov eax, edx
add eax, 0x18
mov eax, DWORD PTR [eax]
push 0x46
call eax
push 0x1
push DWORD PTR [ebp-0x10]
mov ecx, edi
call ebx
inc DWORD PTR [ebp-0xC]
inc DWORD PTR [ebp-0x10]
dec DWORD PTR [ebp-0x8]
jne loop_start
pop ebx
pop edx
pop ecx
pop edi
mov esp, ebp
pop ebp
original_set_event:
push ebp
mov ebp, esp
sub esp, 0x8
jmp 0x0