movabs rcx, OFFSET world_chr_man
mov rcx, QWORD PTR [rcx]
sub rsp, 0x28
lea rdx, [rsp+0x24]
mov DWORD PTR [rdx], OFFSET entity_id
movabs rax, OFFSET fn_chr_ins
call rax
movabs rcx, OFFSET looked_up
mov QWORD PTR [rcx], rax
add rsp, 0x28
ret