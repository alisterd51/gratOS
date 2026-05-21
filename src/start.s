.section .bss
.align 16
stack_bottom:
.skip 16384 # 16 KiB
stack_top:

.section .text
.global _start
.type _start, @function
_start:
	mov $stack_top, %esp

	push %ebx
	push %eax

	call kmain

	cli
1:	hlt
	jmp 1b

.size _start, . - _start

.section .note.GNU-stack,"",@progbits
