sub rsp, 0x28
movabs rcx, OFFSET warp_manager
movabs rdx, OFFSET request_loc
movabs rax, OFFSET fn_request_warp
call rax
add rsp, 0x28
ret