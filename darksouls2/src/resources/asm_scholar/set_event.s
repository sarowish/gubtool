movabs rcx, OFFSET event_flag_man
movabs rdx, OFFSET event_id
mov r8d, OFFSET state
movabs rax, OFFSET fn_set_event
sub rsp, 0x28
call rax
add rsp, 0x28
ret