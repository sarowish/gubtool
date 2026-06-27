movabs rcx, OFFSET current_level
mov ecx, DWORD PTR [rcx]
inc ecx
mov edx, 0x0
movabs r8, OFFSET negative_flag
cmp BYTE PTR [r8], 0x1
je skip_lookup
loop_start:
push rcx
push rdx
movabs r15, OFFSET fn_level_lookup
sub rsp, 0x28
call r15
add rsp, 0x28
pop rdx
pop rcx
add edx, eax
inc ecx
movabs r8, OFFSET new_level
cmp ecx, DWORD PTR [r8]
jle loop_start
skip_lookup:
movabs r8, OFFSET required_souls
mov DWORD PTR [r8], edx
movabs r8, OFFSET current_souls
mov ecx, DWORD PTR [r8]
cmp ecx, edx
jge enough_souls
sub edx, ecx
movabs rcx, OFFSET stats_entity
movabs r15, OFFSET fn_give_souls
sub rsp, 0x48
call r15
add rsp, 0x48
movabs rcx, OFFSET stats_entity
mov ecx, DWORD PTR [rcx+0xec]
movabs rdx, OFFSET current_souls
mov DWORD PTR [rdx], ecx
movabs rdx, OFFSET required_souls
mov edx, DWORD PTR [rdx]
enough_souls:
sub ecx, edx
movabs rcx, OFFSET souls_after
mov DWORD PTR [rcx], ecx
movabs rcx, OFFSET stats_entity
movabs rdx, OFFSET buffer
sub rsp, 0x148
movabs r15, OFFSET fn_level_up
call r15
add rsp, 0x148
ret