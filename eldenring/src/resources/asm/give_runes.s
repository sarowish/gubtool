movabs rcx, OFFSET player_game_data
movabs rdx, OFFSET amount
movabs rax, OFFSET fn_give_runes
sub rsp, 0x28
call rax
add rsp, 0x28
ret