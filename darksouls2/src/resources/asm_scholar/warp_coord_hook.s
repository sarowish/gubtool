.intel_syntax noprefix

subps xmm0, xmm2
movaps XMMWORD PTR [rdi+0x50], xmm0
push rax
movabs rax, 0x0
mov rax, QWORD PTR [rax]
mov rax, QWORD PTR [rax+0x58]
mov rax, QWORD PTR [rax+0x1f8]
mov rax, QWORD PTR [rax+0x18]
mov rax, QWORD PTR [rax+0x8]
add rax, 0x150
cmp rax, rdi
jne exit
movups xmm14, XMMWORD PTR [rip+0x0]
movups XMMWORD PTR [rdi+0x50], xmm14
movups xmm14, XMMWORD PTR [rip+0x0]
movups XMMWORD PTR [rdi+0x60], xmm14
movups xmm14, XMMWORD PTR [rip+0x0]
movups XMMWORD PTR [rdi+0x70], xmm14
movups xmm14, XMMWORD PTR [rip+0x0]
movups XMMWORD PTR [rdi+0x80], xmm14
movups xmm14, XMMWORD PTR [rip+0x0]
xorps xmm14, xmm14
exit:
pop rax
jmp 0x0