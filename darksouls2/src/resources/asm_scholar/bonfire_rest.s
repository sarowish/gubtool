mov edx, OFFSET bonfire_id
movabs rcx, OFFSET bonfire_manager
movabs r14, OFFSET fn_bonfire_rest
sub rsp, 0x28
call r14
add rsp, 0x28
ret