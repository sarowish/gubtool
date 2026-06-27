mov DWORD PTR [edi+0xB8], esi
mov DWORD PTR ds:OFFSET saved_ptr_loc, esi
jmp OFFSET hook_loc