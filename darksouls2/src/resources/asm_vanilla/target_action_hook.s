push ebp
mov ebp, esp
mov eax, DWORD PTR [ebp+0x8]
push edx
push ebx
cmp BYTE PTR ds:OFFSET force_act_flag, 0x1
jne log
mov edx, ds:OFFSET repeating_chr_ai
cmp edx, ecx
jne log
mov eax, ds:OFFSET force_act_id
log:
mov edx, OFFSET buffer
mov ebx, DWORD PTR [edx]
imul ebx, ebx, 0xC
add ebx, edx
add ebx, 0x4
mov DWORD PTR [ebx], ecx
mov DWORD PTR [ebx+0x8], eax
mov ebx, DWORD PTR [edx]
inc ebx
cmp ebx, 0x6
jl skip_reset_idx
mov ebx, 0x0
skip_reset_idx:
mov DWORD PTR [edx], ebx
pop ebx
pop edx
or DWORD PTR [ecx+0x250], 0x1
mov DWORD PTR [ecx+0x25C], eax
pop ebp
ret 0x4