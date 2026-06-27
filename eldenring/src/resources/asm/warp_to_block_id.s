mov ecx, OFFSET area
mov edx, OFFSET block
mov r8d, OFFSET map
mov r9d, OFFSET alt_no
movabs rax, OFFSET fn_block_warp
sub rsp, 0x20
call rax
add rsp, 0x20
ret