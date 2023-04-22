bits 64
section .text
global main
main:
	push	rax
	mov	rax, 0xff
	shl	rax, 56
	not	rax
	mov	BYTE [rax], 1
	pop	rax
	mov	rdi, 0
	ret
