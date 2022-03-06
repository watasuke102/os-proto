extern kernel_main

section .bss
kernel_stack:
	resb	1024*1024

section .text

global kernel_entry
kernel_entry:
	mov	rsp, kernel_stack + 1024*1024
	call	kernel_main
.fin:
	hlt
	jmp	.fin
