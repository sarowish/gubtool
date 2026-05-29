mov ecx, OFFSET event_flag_man
push OFFSET state
push OFFSET event_id
mov eax, OFFSET fn_set_event
call eax
ret