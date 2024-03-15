all:
	mkdir -p .obj/x86
	RUSTFLAGS='--cfg arch__x86 -C soft-float -C panic=abort' cargo +nightly build --release -Z build-std=core --target=arch/x86/target.json
	cp --preserve target/target/release/libkfs_1.a .obj/x86/kernel.a
	i686-elf-as arch/x86/start.s -o .obj/x86/start.o
	i686-elf-ld -o kernel.x86.bin -T arch/x86/link.ld .obj/x86/start.o .obj/x86/kernel.a
	i686-elf-objdump -S kernel.x86.bin > kernel.x86.bin.dsm
