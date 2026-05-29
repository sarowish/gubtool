cmp edx, 0x11
je check_roll
cmp edx, 0x6
je check_jump
cmp edx, 0x10
je check_backstep
normal:
or QWORD PTR [r9+0x10], rax
ret
check_roll:
cmp BYTE PTR [rip+OFFSET roll_flag], 0x1
jne normal
ret
check_jump:
cmp BYTE PTR [rip+OFFSET jump_flag], 0x1
jne normal
ret
check_backstep:
cmp BYTE PTR [rip+OFFSET backstep_flag], 0x1
jne normal
ret