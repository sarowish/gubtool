movabs rcx, OFFSET window_manager
movabs rdx, OFFSET args
movabs r8, OFFSET npc_pos
movabs rax, OFFSET fn_open_menu
sub rsp, 0x28
call rax
add rsp, 0x28
ret