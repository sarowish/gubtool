mov eax, OFFSET amount
push eax
mov ecx, OFFSET stats_entity
mov eax, OFFSET fn_give_souls
call eax
ret