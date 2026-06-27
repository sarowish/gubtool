sub rsp, 0x28
movabs rbx, OFFSET item_struct
movabs rcx, OFFSET check_quantity_flag
cmp BYTE PTR [rcx], 0x0
je skip_quantity_adjust
lea rcx, [rbx+0x44]
movabs rax, OFFSET fn_get_item_quantity
call rax
movabs rdi, OFFSET max_quantity
mov edi, DWORD PTR [rdi]
mov edx, DWORD PTR [rbx+0x48]
add edx, eax
cmp edx, edi
jle skip_quantity_adjust
sub edi, eax
mov DWORD PTR [rbx+0x48], edi
skip_quantity_adjust:
movabs rcx, OFFSET map_item_man_impl
mov rcx, QWORD PTR [rcx]
lea rdx, [rbx+0x40]
lea r8, [rbx+0xe4]
xor r9d, r9d
movabs rax, OFFSET fn_item_spawn
call rax
add rsp, 0x28
ret