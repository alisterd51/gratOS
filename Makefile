BUILD	:= debug
# BUILD	:= release
ARCH	:= x86
BOOTLOADER	:= grub
# BOOTLOADER	:= limine
BOOTPROTOCOL	:= multiboot
# BOOTPROTOCOL	:= multiboot2

# utlis
CARGO	:= cargo
QEMU	:= qemu-system-i386

# files
LINKSCRIPT	:= arch/$(ARCH)/link.ld
ISO			:= gratos.${ARCH}.${BOOTLOADER}.iso
GRUBCFG		:= boot/grub-${BOOTPROTOCOL}.cfg
LIMINECONF	:= boot/limine-${BOOTPROTOCOL}.conf

# flags
CARGOFLAGS.debug	:= --features=${BOOTPROTOCOL}
CARGOFLAGS.release	:= --features=${BOOTPROTOCOL} --release

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

limine-binary:
	curl -sL https://github.com/Limine-Bootloader/Limine/releases/download/v12.3.1/limine-binary.tar.xz | tar xJf -

.PHONY: target/target/${BUILD}/gratos
target/target/${BUILD}/gratos: $(shell find src -type f -name '*.rs') $(shell find src -type f -name '*.s') Cargo.toml
	${CARGO} build ${CARGOFLAGS.${BUILD}}
	grub-file --is-${ARCH}-${BOOTPROTOCOL} target/target/${BUILD}/gratos

${BUILDDIR}/gratos.${ARCH}.grub.iso: target/target/${BUILD}/gratos
	mkdir -p ${ISODIR}/boot/grub
	cp target/target/${BUILD}/gratos ${ISODIR}/boot/${BIN}
	cp ${GRUBCFG} ${ISODIR}/boot/grub/grub.cfg
	grub-mkrescue --compress=xz -o ${BUILDDIR}/${ISO} ${ISODIR}

${BUILDDIR}/gratos.${ARCH}.limine.iso: target/target/${BUILD}/gratos limine-binary
	make -C limine-binary
	mkdir -p ${ISODIR}/boot/limine
	cp -v target/target/${BUILD}/gratos ${ISODIR}/boot/
	cp -v ${LIMINECONF} ${ISODIR}/boot/limine/limine.conf
	cp -v limine-binary/limine-bios.sys limine-binary/limine-bios-cd.bin limine-binary/limine-uefi-cd.bin ${ISODIR}/boot/limine/
	mkdir -p ${ISODIR}/EFI/BOOT
	cp -v limine-binary/BOOTX64.EFI ${ISODIR}/EFI/BOOT/
	cp -v limine-binary/BOOTIA32.EFI ${ISODIR}/EFI/BOOT/
	xorriso -as mkisofs -R -r -J -b boot/limine/limine-bios-cd.bin \
        -no-emul-boot -boot-load-size 4 -boot-info-table -hfsplus \
        -apm-block-size 2048 --efi-boot boot/limine/limine-uefi-cd.bin \
        -efi-boot-part --efi-boot-image --protective-msdos-label \
        ${ISODIR} -o ${BUILDDIR}/${ISO}
	./limine-binary/limine bios-install ${BUILDDIR}/${ISO}

.PHONY: clean
clean:
	${CARGO} clean
	rm -rf build limine-binary

.PHONY: run
run: ${BUILDDIR}/${ISO}
	${QEMU} -cdrom $<
