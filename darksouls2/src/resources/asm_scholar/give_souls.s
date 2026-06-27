mov rdx, OFFSET amount
movabs rcx, OFFSET stats_entity
movabs rax, OFFSET fn_give_souls
sub rsp, 0x48
call rax
add rsp, 0x48
ret