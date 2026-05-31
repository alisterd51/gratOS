.section .text

.macro isr_no_error_code num
.global isr\num
isr\num:
    pushl $0
    pushl $\num
    jmp isr_common
.endm

.macro isr_error_code num
.global isr\num
isr\num:
    pushl $\num
    jmp isr_common
.endm

isr_no_error_code 0  /* Divide Error */
isr_no_error_code 1  /* Debug Exception */
isr_no_error_code 2  /* NMI Interrupt */
isr_no_error_code 3  /* Breakpoint */
isr_no_error_code 4  /* Overflow */
isr_no_error_code 5  /* BOUND Range Exceeded */
isr_no_error_code 6  /* Invalid Opcode */
isr_no_error_code 7  /* Device Not Available */
isr_error_code    8  /* Double Fault */
isr_no_error_code 9  /* Coprocessor Segment Overrun */
isr_error_code    10 /* Invalid TSS */
isr_error_code    11 /* Segment Not Present */
isr_error_code    12 /* Stack-Segment Fault */
isr_error_code    13 /* General Protection */
isr_error_code    14 /* Page Fault */
isr_no_error_code 15 /* Reserved */
isr_no_error_code 16 /* x87 FPU Floating-Point Error */
isr_error_code    17 /* Alignment Check */
isr_no_error_code 18 /* Machine Check */
isr_no_error_code 19 /* SIMD Floating-Point Exception */
isr_no_error_code 20 /* Virtualization Exception */
isr_error_code    21 /* Control Protection Exception */
isr_no_error_code 22 /* Reserved */
isr_no_error_code 23 /* Reserved */
isr_no_error_code 24 /* Reserved */
isr_no_error_code 25 /* Reserved */
isr_no_error_code 26 /* Reserved */
isr_no_error_code 27 /* Reserved */
isr_no_error_code 28 /* Reserved */
isr_no_error_code 29 /* Reserved */
isr_no_error_code 30 /* Reserved */
isr_no_error_code 31 /* Reserved */
isr_no_error_code 32 /* Programmable Interrupt Timer Interrupt */
isr_no_error_code 33 /* Keyboard Interrupt */
isr_no_error_code 34 /* Cascade */
isr_no_error_code 35 /* COM2 */
isr_no_error_code 36 /* COM1 */
isr_no_error_code 37 /* LPT2 */
isr_no_error_code 38 /* Floppy Disk */
isr_no_error_code 39 /* LPT1 */
isr_no_error_code 40 /* CMOS real-time clock */
isr_no_error_code 41 /* Free for peripherals / legacy SCSI / NIC */
isr_no_error_code 42 /* Free for peripherals / SCSI / NIC */
isr_no_error_code 43 /* Free for peripherals / SCSI / NIC */
isr_no_error_code 44 /* PS2 Mouse */
isr_no_error_code 45 /* FPU / Coprocessor / Inter-processor */
isr_no_error_code 46 /* Primary ATA Hard Disk */
isr_no_error_code 47 /* Secondary ATA Hard Disk */

.extern interrupt_dispatcher

isr_common:
    pushal

    pushl %ds
    pushl %es
    pushl %fs
    pushl %gs

    movw $0x10, %ax
    movw %ax, %ds
    movw %ax, %es
    movw %ax, %fs
    movw %ax, %gs

    pushl %esp
    call interrupt_dispatcher
    addl $4, %esp

    popl %gs
    popl %fs
    popl %es
    popl %ds
    popal

    addl $8, %esp

    iret
