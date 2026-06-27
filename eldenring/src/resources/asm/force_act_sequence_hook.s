push r9
movabs r9, OFFSET should_run_flag
cmp BYTE PTR [r9], 0x1
jne normal
cmp dword ptr [rax+0x28], OFFSET npc_think_param_id
jne normal
push rcx
push rdx
push r8
movabs rcx, OFFSET current_idx
mov edx, DWORD PTR [rcx]
movabs r8, OFFSET act_array
mov eax, DWORD PTR [r8+rdx*4]
inc edx
mov DWORD PTR [rcx], edx
cmp edx, 0xB
jne skip_set_flag
mov BYTE PTR [r9], 0x0
skip_set_flag:
pop r8
pop rdx
pop rcx
pop r9
jmp return
normal:
pop r9
movsx eax, BYTE PTR [rax+OFFSET orig_instr_off]
return:
jmp OFFSET hook_loc