mov edx, OFFSET bonfire_id
mov ecx, OFFSET bonfire_manager
mov eax, OFFSET fn_bonfire_rest
push edx
call eax
ret