movabs rcx, OFFSET virt_mem_flag
movabs rdx, OFFSET event_id
movabs r8, OFFSET state
movabs rax, OFFSET fn_set_event
sub rsp, 0x20
call rax
add rsp, 0x20
ret