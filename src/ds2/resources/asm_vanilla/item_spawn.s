.intel_syntax noprefix

main:
cmp BYTE PTR ds:0x0, 0x1
jne nothing_to_process
mov ebx, DWORD PTR ds:0x0
mov edi, ebx
mov ebx, DWORD PTR [ebx+0x60]
mov ebx, DWORD PTR [ebx+0x8]
mov ebx, DWORD PTR [ebx+0x8]
mov BYTE PTR ds:0x0, 0x0
cmp BYTE PTR ds:0x0, 0x1
jne skip_adjust
mov ecx, DWORD PTR [ebx+0x8]
mov edx, DWORD PTR ds:0x0
push edx
lea edx, ds:0x0
push edx
lea edx, ds:0x0
push edx
call 0x0
movzx eax, WORD PTR ds:0x0
add eax, DWORD PTR ds:0x0
cmp eax, DWORD PTR ds:0x0
jle skip_adjust
mov eax, ds:0x0
sub eax, DWORD PTR ds:0x0
mov ds:0x0, ax
skip_adjust:
mov ecx, ebx
push 0x0
push DWORD PTR ds:0x0
lea edx, ds:0x0
push edx
call 0x0
push 0x1
push DWORD PTR ds:0x0
lea eax, ds:0x0
push eax
lea eax, ds:0x0
push eax
call 0x0
add esp, 0x10
mov ecx, DWORD PTR [edi+0xCC4]
lea eax, ds:0x0
push eax
call 0x0
nothing_to_process:
mov eax, ds:0x0
push 0x5
call eax
cmp BYTE PTR ds:0x0, 0x1
jne main
ret