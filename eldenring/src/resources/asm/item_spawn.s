sub rsp, 0x28
lea rbx, [rip+0x0]
cmp BYTE PTR [rip+0x0], 0x0
je skip_quantity_adjust
lea rcx, [rbx+0x44]
call 0x0
mov edi, DWORD PTR [rip+0x0]
mov edx, DWORD PTR [rbx+0x48]
add edx, eax
cmp edx, edi
jle skip_quantity_adjust
sub edi, eax
mov DWORD PTR [rbx+0x48], edi
skip_quantity_adjust:
mov rcx, QWORD PTR [rip+0x0]
lea rdx, [rbx+0x40]
lea r8, [rbx+0xe4]
xor r9d, r9d
call 0x0
add rsp, 0x28
ret