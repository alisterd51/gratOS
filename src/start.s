.section .bss
.align 16
stack_bottom:
.skip 16384 # 16 KiB
stack_top:

.section .text
.global _start
.type _start, @function
_start:
    mov esp, offset stack_top

    push ebx
    push eax

    call kmain

    cli
1:  hlt
    jmp 1b

.size _start, . - _start

.section .note.GNU-stack,"",@progbits
