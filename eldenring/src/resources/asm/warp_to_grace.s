movabs rax, OFFSET world_chr_man
mov rax, [rax]
mov rcx, QWORD PTR [rax+0x18]
mov rdx, QWORD PTR [rax+0x08]
movabs r8, OFFSET grace_id
movabs rax, OFFSET fn_grace_warp
sub rsp, 0x28
call rax
add rsp, 0x28
ret