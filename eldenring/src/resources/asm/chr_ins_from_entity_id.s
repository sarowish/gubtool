.intel_syntax noprefix

movabs rcx, 0x0
lea rdx, [rsp+0x24]
mov DWORD PTR [rdx], 0x0
movabs rax,0x0
sub rsp, 0x28
call rax
movabs rax, 0x0
mov QWORD PTR [rcx], rax
add rsp, 0x28
ret