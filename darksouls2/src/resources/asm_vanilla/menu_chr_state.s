mov ecx, OFFSET dl_back_allocator
push OFFSET state
mov eax, OFFSET fn_menu_chr_state
push 0x11
call eax
ret