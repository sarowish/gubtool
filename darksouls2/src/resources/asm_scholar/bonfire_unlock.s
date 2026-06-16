movabs rcx, OFFSET bonfire_manager
mov edx, OFFSET bonfire_id
movabs rax, OFFSET fn_bonfire_unlock
mov r8b, 0x1 # show popup
sub rsp, 0x28
call rax
add rsp, 0x28
ret