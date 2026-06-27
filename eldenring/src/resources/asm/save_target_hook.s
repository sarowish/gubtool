push r8
mov rcx, [rdi+0x88]
movabs r8, OFFSET saved_pointer_loc
mov [r8], rax
pop r8
jmp OFFSET hook_loc