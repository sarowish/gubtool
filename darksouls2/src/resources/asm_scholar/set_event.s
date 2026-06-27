movabs rcx, OFFSET event_flag_man
mov r8d, OFFSET state
mov rdx, OFFSET event_id
movabs rax, OFFSET fn_set_event
sub rsp, 0x28
call rax
add rsp, 0x28
ret