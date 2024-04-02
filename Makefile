# configuration
    TRIPLE := i686-elf
      ARCH := x86
      NAME := gratos
BUILD_MODE := ${if ${RELEASE},release,debug}

# commands
        MKDIR := ${shell which mkdir}
           AS := ${shell which ${TRIPLE}-as}
        CARGO := ${shell which cargo}
           LD := ${shell which ${TRIPLE}-ld}
    GRUB_FILE := ${shell which grub-file}
GRUB_MKRESCUE := ${shell which grub-mkrescue}
         QEMU := ${shell which qemu-system-x86_64}
           RM := ${shell which rm}

# directories
       ARCH_DIR := arch/${ARCH}
        OBJ_DIR := obj
        ISO_DIR := iso
       BOOT_DIR := ${ISO_DIR}/boot
CARGO_BUILD_DIR := target/target/${BUILD_MODE}

# files
     BIN := ${BOOT_DIR}/${NAME}.${ARCH}.bin
     ISO := ${NAME}.${ARCH}.iso
 ASM_SRC := \
${addsuffix .s, \
	start \
}
 ASM_OBJ := ${addprefix ${OBJ_DIR}/${BUILD_MODE}/, ${ASM_SRC:.s=.o}}
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

# rules
.PHONY: all run clean re

all: ${ISO}

${ISO}: ${BIN}
	${GRUB_MKRESCUE} --compress=xz ${OUTPUT_OPTION} ${ISO_DIR}

${BIN}: ${ASM_OBJ} ${LIB}
	${LD} -T ${ARCH_DIR}/link.ld ${OUTPUT_OPTION} $^
	${GRUB_FILE} --is-${ARCH}-multiboot $@

${OBJ_DIR}/${BUILD_MODE}/%.o: ${ARCH_DIR}/%.s
	${MKDIR} -p ${@D}
	${AS}${if ${RELEASE},, -g} ${OUTPUT_OPTION} $<

${LIB}: ${RUST_SRC}
	${CARGO} build${if ${RELEASE}, --release}

run: ${ISO}
	${QEMU} -cdrom $<

clean:
	${CARGO} clean
	${RM} -rf ${OBJ_DIR} ${BIN} ${ISO}

re: clean all
