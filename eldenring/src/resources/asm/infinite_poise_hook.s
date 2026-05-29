mov r8, rdi
movzx edx, bpl
push rax
mov rax, QWORD PTR [rip+0x0]
cmp rcx, QWORD PTR [rax+0x1E508]
je invalidatePoise
mov rax, QWORD PTR [rax+0x1E508]
mov rax, QWORD PTR [rax+0x190]
mov rax, QWORD PTR [rax+0xE8]
mov rax, QWORD PTR [rax+0x10]
cmp DWORD PTR [rax+0x50], 0x0
je exit
push rcx
push rdx
push r8
push r9
mov rcx, QWORD PTR [rip+0x0]
sub rsp, 0x28
lea rdx, [rsp+0x24]
mov DWORD PTR [rdx], 0x9C40
call 0x0
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
pop rax
jmp 0x0