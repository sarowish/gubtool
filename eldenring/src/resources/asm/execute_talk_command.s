mov rbp, rsp
sub rsp, 0x598
lea rcx, [rbp-0x420]
mov edx, OFFSET command_id
movabs rax, OFFSET fn_external_event_temp_ctor
call rax
lea rcx, [rbp-0x438]
lea rdx, [rbp-0x538]
mov QWORD PTR [rcx+0x10], rdx
lea r8, [rbp-0x558]
mov QWORD PTR [rdx+0x98], r8
mov QWORD PTR [r8+0x18], rdx
movabs rax, OFFSET chr_handle
mov QWORD PTR [rdx+0x40], rax
mov ebx, OFFSET params_len
mov DWORD PTR [rbp-0x3e0], ebx
xor edi, edi
movabs rsi, OFFSET params_loc
param_loop:
mov eax, DWORD PTR [rsi+rdi*4]
lea r8, [rbp-0x568]
mov DWORD PTR [r8], eax
mov DWORD PTR [r8+0x8], 0x2
lea edx, [rdi+0x1]
lea rcx, [rbp-0x420]
mov rax, QWORD PTR [rcx]
call QWORD PTR [rax+0x8]
inc edi
cmp edi, ebx
jl param_loop
lea rcx, [rbp-0x438]
lea rdx, [rbp-0x420]
movabs rax, OFFSET fn_execute_talk_command
call rax
add rsp, 0x598
ret