mov ecx, OFFSET bonfire_manager
mov edx, OFFSET bonfire_id
mov eax, OFFSET fn_bonfire_rest
push edx
call eax
ret