push rax
push r9
push r10
movabs rax, OFFSET force_act_flag
cmp BYTE PTR [rax], 0x1
jne log
movabs rax, OFFSET repeating_chr_ai
mov rax, QWORD PTR [rax]
cmp rax, rcx
jne log
movabs rax, OFFSET force_act_id
mov edx, DWORD PTR [rax]
log:
movabs rax, OFFSET buffer
mov r9d, DWORD PTR [rax]
mov r10, r9
imul r10, r10, 0xC
add r10, rax
add r10, 0x4
mov QWORD PTR [r10], rcx
mov DWORD PTR [r10+0x8], edx
inc r9
cmp r9, 0x6
jl skip_reset_idx
mov r9, 0x0
skip_reset_idx:
mov DWORD PTR [rax], r9d
pop r10
pop r9
pop rax
or DWORD PTR [rcx+0x350], 0x1
mov DWORD PTR [rcx+0x35C], edx
ret