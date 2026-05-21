.set MAGIC, 0xE85250D6
.set ARCH, 0
.set HEADER_LENGTH, header_end - header_start
.set CHECKSUM, -(MAGIC + ARCH + HEADER_LENGTH)

.section .multiboot
.align 8
header_start:
    .long MAGIC
    .long ARCH
    .long HEADER_LENGTH
    .long CHECKSUM

    .short 0
    .short 0
    .long 8
header_end:
