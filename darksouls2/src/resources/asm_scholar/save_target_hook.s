push rax
movabs rax, OFFSET saved_ptr_loc
mov QWORD PTR [rax], rdi
mov QWORD PTR [rbx+0xC0], rdi
pop rax
jmp OFFSET hook_loc