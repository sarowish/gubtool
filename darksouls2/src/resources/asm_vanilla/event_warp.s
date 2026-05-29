mov ecx, OFFSET event_warp_entity
lea eax, ds: OFFSET params_location
push eax
mov eax, OFFSET fn_warp
call eax
ret