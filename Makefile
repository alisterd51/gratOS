# configuration
TRIPLE := i686-elf
  ARCH := x86
  NAME := gratos

# commands
           AS := ${shell which ${TRIPLE}-as}
           LD := ${shell which ${TRIPLE}-ld}
      OBJDUMP := ${shell which ${TRIPLE}-objdump}
        CARGO := ${shell which cargo}
         QEMU := ${shell which qemu-system-x86_64}
        MKDIR := ${shell which mkdir} -p
           CP := ${shell which cp}
           RM := ${shell which rm} -rf
    GRUB_FILE := ${shell which grub-file}
GRUB_MKRESCUE := ${shell which grub-mkrescue}

# directories
       ARCH_DIR := arch/${ARCH}
        OBJ_DIR := obj
        ISO_DIR := iso
       BOOT_DIR := ${ISO_DIR}/boot
CARGO_BUILD_DIR := target/target/debug

# files
     BIN := ${NAME}.${ARCH}.bin
     ISO := ${NAME}.${ARCH}.iso
 ASM_SRC := \
${addsuffix .s, \
	start \
}
 ASM_OBJ := ${addprefix ${OBJ_DIR}/, ${ASM_SRC:.s=.o}}
RUST_SRC := \
${addsuffix .rs, \
	${addprefix src/, \
		io \
		lib \
		vga_buffer \
	} \
}
     LIB := ${CARGO_BUILD_DIR}/lib${NAME}.a
GRUB_CFG := ${BOOT_DIR}/grub/grub.cfg

# flags
GRUB_MKRESCUE_FLAGS := --compress=xz

# rules
.PHONY: all run clean re

all: ${ISO}

${OBJ_DIR}/%.o: ${ARCH_DIR}/%.s
	${MKDIR} ${OBJ_DIR}
	${AS} $< ${OUTPUT_OPTION}

${LIB}: ${RUST_SRC}
	${CARGO} +nightly build

${BIN}: ${ASM_OBJ} ${LIB}
	${LD} -T ${ARCH_DIR}/link.ld ${OUTPUT_OPTION} $^
	${GRUB_FILE} --is-${ARCH}-multiboot ${BIN}

${ISO}: ${BIN}
	${CP} $< ${BOOT_DIR}/$<
	${GRUB_MKRESCUE} ${GRUB_MKRESCUE_FLAGS} ${OUTPUT_OPTION} ${ISO_DIR}

run: ${ISO}
	${QEMU} -cdrom $<

clean:
	${CARGO} clean
	${RM} ${OBJ_DIR} ${BOOT_DIR}/${BIN} ${BIN} ${ISO}

re: clean all
