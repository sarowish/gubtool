movabs rax, ds:0x0
mov rcx, QWORD PTR [rax+0x18]
mov rdx, QWORD PTR [rax+0x08]
movabs r8, 0x0
movabs rax, 0x0
sub rsp, 0x28
call rax
add rsp, 0x28
ret