movabs r12, OFFSET bonfire_manager
movabs r14, OFFSET fn_bonfire_unlock
mov r13d, DWORD PTR [r12+0x28]
test r13d, r13d
je done
mov rbx, QWORD PTR [r12+0x20]
xor esi, esi
sub rsp, 0x28
loop_start:
mov rax, rsi
imul rax, rax, 0x18
add rax, rbx
movzx edx, WORD PTR [rax]
mov rcx, r12
mov r8b, 0x1
call r14
inc esi
cmp esi, r13d
jl loop_start
add rsp, 0x28
done:
ret