BUILD	:= debug
ARCH	:= x86

# utlis
CARGO	:= cargo
QEMU	:= qemu-system-i386

# files
LINKSCRIPT	:= arch/$(ARCH)/link.ld
ISO			:= gratos.${ARCH}.iso
GRUBCFG		:= grub/grub.cfg

# flags
CARGOFLAGS.debug	:=
CARGOFLAGS.release	:= --release

# dirs
BUILDDIR	:= build/${BUILD}
ISODIR		:= ${BUILDDIR}/isodir

.PHONY: all
all: iso

.PHONY: bin
bin: ${BUILDDIR}/${BIN}

.PHONY: iso
iso: ${BUILDDIR}/${ISO}

${BUILDDIR}:
	mkdir -p $@

target/target/${BUILD}/gratos: $(shell find src -type f -name '*.rs') $(shell find src -type f -name '*.s') Cargo.toml
	${CARGO} build ${CARGOFLAGS.${BUILD}}
	grub-file --is-${ARCH}-multiboot target/target/${BUILD}/gratos

${BUILDDIR}/${ISO}: target/target/${BUILD}/gratos
	mkdir -p ${ISODIR}/boot/grub
	cp target/target/${BUILD}/gratos ${ISODIR}/boot/${BIN}
	cp ${GRUBCFG} ${ISODIR}/boot/grub/grub.cfg
	grub-mkrescue --compress=xz -o ${BUILDDIR}/${ISO} ${ISODIR}

.PHONY: clean
clean:
	${CARGO} clean
	rm -rf build

.PHONY: run
run: ${BUILDDIR}/${ISO}
	${QEMU} -cdrom $<
