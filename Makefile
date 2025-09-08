BUILD	:= debug
# BUILD	:= release
ARCH	:= x86
BOOTLOADER	:= grub
# BOOTLOADER	:= limine

# utlis
CARGO	:= cargo
QEMU	:= qemu-system-i386

# files
LINKSCRIPT	:= arch/$(ARCH)/link.ld
ISO			:= gratos.${ARCH}.${BOOTLOADER}.iso
GRUBCFG		:= grub.cfg
LIMINECONF	:= limine.conf

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

limine:
	git clone https://codeberg.org/Limine/Limine.git --branch=v9.x-binary --depth=1 limine

target/target/${BUILD}/gratos: $(shell find src -type f -name '*.rs') $(shell find src -type f -name '*.s') Cargo.toml
	${CARGO} build ${CARGOFLAGS.${BUILD}}
	grub-file --is-${ARCH}-multiboot target/target/${BUILD}/gratos

${BUILDDIR}/gratos.${ARCH}.grub.iso: target/target/${BUILD}/gratos
	mkdir -p ${ISODIR}/boot/grub
	cp target/target/${BUILD}/gratos ${ISODIR}/boot/${BIN}
	cp ${GRUBCFG} ${ISODIR}/boot/grub/grub.cfg
	grub-mkrescue --compress=xz -o ${BUILDDIR}/${ISO} ${ISODIR}

${BUILDDIR}/gratos.${ARCH}.limine.iso: target/target/${BUILD}/gratos limine
	make -C limine
	mkdir -p ${ISODIR}/boot/limine
	cp -v target/target/${BUILD}/gratos ${ISODIR}/boot/
	cp -v ${LIMINECONF} limine/limine-bios.sys limine/limine-bios-cd.bin limine/limine-uefi-cd.bin ${ISODIR}/boot/limine/
	mkdir -p ${ISODIR}/EFI/BOOT
	cp -v limine/BOOTX64.EFI ${ISODIR}/EFI/BOOT/
	cp -v limine/BOOTIA32.EFI ${ISODIR}/EFI/BOOT/
	xorriso -as mkisofs -R -r -J -b boot/limine/limine-bios-cd.bin \
        -no-emul-boot -boot-load-size 4 -boot-info-table -hfsplus \
        -apm-block-size 2048 --efi-boot boot/limine/limine-uefi-cd.bin \
        -efi-boot-part --efi-boot-image --protective-msdos-label \
        ${ISODIR} -o ${BUILDDIR}/${ISO}
	./limine/limine bios-install ${BUILDDIR}/${ISO}

.PHONY: clean
clean:
	${CARGO} clean
	rm -rf build limine

.PHONY: run
run: ${BUILDDIR}/${ISO}
	${QEMU} -cdrom $<
