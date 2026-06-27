movabs rcx, OFFSET speffect_ptr
movabs rdx, OFFSET speffect_id
movabs rax, OFFSET fn_remove_speffect
sub rsp, 0x20
call rax
add rsp, 0x20
ret