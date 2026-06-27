movabs r8, OFFSET npc_pos
movabs rdx, OFFSET args
movabs rcx, OFFSET window_manager
movabs rax, OFFSET fn_open_menu
sub rsp, 0x28
call rax
add rsp, 0x28
ret