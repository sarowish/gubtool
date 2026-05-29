sub rsp, 0x118
movabs rcx, OFFSET event_warp_entity
lea rdx, [rip+OFFSET params_location]
call OFFSET fn_warp
add rsp, 0x118
ret