push OFFSET npc_pos
push OFFSET args
mov ecx, OFFSET window_manager
mov eax, OFFSET fn_open_menu
call eax
ret