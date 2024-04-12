TRIPLE	:= i686-elf

# utlis
AS		:= ${TRIPLE}-as
LD		:= ${TRIPLE}-ld
OBJDUMP	:= ${TRIPLE}-objdump
CARGO	:= cargo
QEMU	:= qemu-system-x86_64

ARCH	:= x86

LINKSCRIPT	:= arch/$(ARCH)/link.ld
TARGETFILE	:= arch/$(ARCH)/target.json
BIN		:= kernel.${ARCH}.bin
ISO		:= kernel.${ARCH}.iso
GRUBCFG	:= grub/grub.cfg

# flags
ASFLAGS		:=
LDFLAGS		:= --script ${LINKSCRIPT}
CARGOFLAGS	:= --target=${TARGETFILE}


OBJDIR	:= .obj/${ARCH}
ISODIR	:= isodir

COMPILE.AS	= ${AS} ${ASFLAGS} $< -o $@
LINK		= ${LD} ${LDFLAGS} ${OBJDIR}/start.o ${OBJDIR}/kernel.a -o $@

all: ${ISO}

${BIN}:
	mkdir -p ${OBJDIR}
	${CARGO} build --release
	cp --preserve target/target/release/libgratos.a ${OBJDIR}/kernel.a
	${AS} arch/${ARCH}/start.s -o ${OBJDIR}/start.o
	${LD} -o ${BIN} -T arch/${ARCH}/link.ld ${OBJDIR}/start.o ${OBJDIR}/kernel.a
	grub-file --is-${ARCH}-multiboot ${BIN}

${ISO}: ${BIN}
	mkdir -p ${ISODIR}/boot/grub
	cp ${BIN} ${ISODIR}/boot/${BIN}
	cp ${GRUBCFG} ${ISODIR}/boot/grub/grub.cfg
	grub-mkrescue --compress=xz -o ${ISO} ${ISODIR}

clean:
	${CARGO} clean
	rm -rf ${OBJDIR} ${ISODIR} ${BIN} ${ISO}

run: ${ISO}
	${QEMU} -cdrom ${ISO}
