mov r8, rdi
movzx edx, bpl
push rax
push r14
push r15
movabs r14, OFFSET world_chr_man
mov r15, OFFSET player_ins_off
mov rax, QWORD PTR [r14]
cmp rcx, QWORD PTR [rax+r15]
je invalidatePoise
mov rax, QWORD PTR [rax+r15]
mov rax, QWORD PTR [rax+0x190]
mov rax, QWORD PTR [rax+0xE8]
mov rax, QWORD PTR [rax+0x10]
cmp DWORD PTR [rax+0x50], 0x0
je exit
push rcx
push rdx
push r8
push r9
mov rcx, QWORD PTR [r14]
sub rsp, 0x28
lea rdx, [rsp+0x24]
mov DWORD PTR [rdx], 0x9C40
movabs r8, OFFSET fn_get_chr_ins
call r8
add rsp,0x28
pop r9
pop r8
pop rdx
pop rcx
cmp rax, rcx
jne exit
invalidatePoise:
xor edx, edx
exit:
pop r15
pop r14
pop rax
jmp OFFSET hook_loc