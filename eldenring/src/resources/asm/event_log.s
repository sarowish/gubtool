mov QWORD PTR [rsp+0x8], rbx
push rax
push rbx
push rdi
mov edi, DWORD PTR [rip+OFFSET write_index_1]
mov eax, edi
imul eax, eax,0x5
lea rbx, [rip+OFFSET buffer]
add rbx, rax
mov DWORD PTR [rbx], edx
mov BYTE PTR [rbx+0x4], r8b
inc edi
and edi, 0x1FF
mov DWORD PTR [rip+OFFSET write_index_2], edi
pop rdi
pop rbx
pop rax
jmp OFFSET hook_loc