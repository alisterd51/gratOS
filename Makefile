# utlis
AS		:= i686-elf-as
LD		:= i686-elf-ld
OBJDUMP	:= i686-elf-objdump
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
RUSTFLAGS	:= --cfg arch__x86 -C soft-float -C panic=abort

OBJDIR	:= .obj/${ARCH}
ISODIR	:= isodir

COMPILE.AS	= ${AS} ${ASFLAGS} $< -o $@
LINK		= ${LD} ${LDFLAGS} ${OBJDIR}/start.o ${OBJDIR}/kernel.a -o $@

all: ${ISO}

${BIN}:
	mkdir -p ${OBJDIR}
	RUSTFLAGS='${RUSTFLAGS}' ${CARGO} +nightly build --release -Z build-std=core,compiler_builtins -Z build-std-features=compiler-builtins-mem --target=${TARGETFILE}
	cp --preserve target/target/release/libkfs_1.a ${OBJDIR}/kernel.a
	${AS} arch/x86/start.s -o ${OBJDIR}/start.o
	${LD} -o ${BIN} -T arch/x86/link.ld ${OBJDIR}/start.o ${OBJDIR}/kernel.a
	grub-file --is-x86-multiboot ${BIN}

${ISO}: ${BIN}
	mkdir -p ${ISODIR}/boot/grub
	cp ${BIN} ${ISODIR}/boot/${BIN}
	cp ${GRUBCFG} ${ISODIR}/boot/grub/grub.cfg
	grub-mkrescue -o ${ISO} ${ISODIR}

clean:
	${CARGO} clean
	rm -rf ${OBJDIR} ${ISODIR} ${BIN} ${ISO}

run: ${ISO}
	${QEMU} -cdrom ${ISO}
