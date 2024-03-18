# KFS-1 (provisional name)

## Presentation

Kernel from scratch (in rust)

## Run

### Run from bin

```bash
make
qemu-system-x86_64 -kernel kernel.x86.bin
```

### Run from iso (todo)

## TODO

- [ ] Mandatory:
  - [x] Install GRUB on an virtual image
  - [x] Write an ASM boot code that handles multiboot header, and use GRUB to init and call main function of the kernel itself
  - [x] Write basic kernel code of the choosen language.
  - [x] Compile it with correct flags, and link it to make it bootable.
  - [ ] Once all of those steps above are done, you can write some helpers like kernel types or basic functions (strlen, strcmp, ...)
  - [ ] Your work must not exceed 10 MB.
  - [ ] Code the interface between your kernel and the screen.
  - [ ] Display "42" on the screen.
    - [ ] clean screen
    - [ ] print "42"
  - [ ] Makefile:
    Your makefile must compile all your source files with the right flags and the right compiler. Keep in mind that your kernel will use at least two different languages (ASM and whatever-you-choose), so make (<- joke) your Makefile’s rules correctly
- [ ] Bonus:
  - [ ] Add scroll and cursor support to your I/O interface.
  - [ ] Add colors support to your I/O interface.
  - [ ] Add helpers like printf / printk in order to print information / debug easily
    - [ ] print!
    - [ ] println!
  - [ ] Handle keyboard entries and print them.
  - [ ] Handle different screens, and keyboard shortcuts to switch easily between then.
- [ ] Other:
  - [ ] add CI to build and serve an iso of kfs
