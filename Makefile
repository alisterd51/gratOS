TRIPLE	:= i686-elf
BUILD	:= debug
ARCH	:= x86

# utlis
AS		:= ${TRIPLE}-as
LD		:= ${TRIPLE}-ld
OBJDUMP	:= ${TRIPLE}-objdump
CARGO	:= cargo
QEMU	:= qemu-system-x86_64

# files
LINKSCRIPT	:= arch/$(ARCH)/link.ld
BIN			:= gratos.${ARCH}.bin
ISO			:= gratos.${ARCH}.iso
GRUBCFG		:= grub/grub.cfg

# flags
ASFLAGS				:=
LDFLAGS				:= --script ${LINKSCRIPT}
CARGOFLAGS.debug	:=
CARGOFLAGS.release	:= --release

# dirs
BUILDDIR	:= build/${BUILD}
OBJDIR		:= ${BUILDDIR}/obj/${ARCH}
ISODIR		:= ${BUILDDIR}/isodir

.PHONY: all
all: iso

.PHONY: bin
bin: ${BUILDDIR}/${BIN}

.PHONY: iso
iso: ${BUILDDIR}/${ISO}

${BUILDDIR}:
	mkdir -p $@

${OBJDIR}:
	mkdir -p $@

${OBJDIR}/%.o: arch/${ARCH}/%.s ${OBJDIR}
	${AS} $< -o $@

target/target/${BUILD}/libgratos.a: $(shell find src -type f -name '*.rs') Cargo.toml
	${CARGO} build ${CARGOFLAGS.${BUILD}}

${BUILDDIR}/${BIN}: ${OBJDIR} ${OBJDIR}/start.o target/target/${BUILD}/libgratos.a
	${LD} -o ${BUILDDIR}/${BIN} -T arch/${ARCH}/link.ld ${OBJDIR}/start.o target/target/${BUILD}/libgratos.a
	grub-file --is-${ARCH}-multiboot ${BUILDDIR}/${BIN}

${BUILDDIR}/${ISO}: ${BUILDDIR}/${BIN}
	mkdir -p ${ISODIR}/boot/grub
	cp ${BUILDDIR}/${BIN} ${ISODIR}/boot/${BIN}
	cp ${GRUBCFG} ${ISODIR}/boot/grub/grub.cfg
	grub-mkrescue -o ${BUILDDIR}/${ISO} ${ISODIR}

.PHONY: clean
clean:
	${CARGO} clean
	rm -rf build

.PHONY: run
run: ${BUILDDIR}/${ISO}
	${QEMU} -cdrom $<
