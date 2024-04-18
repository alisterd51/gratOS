# gratOS

[![Nightly Release](https://github.com/alisterd51/gratOS/actions/workflows/nightly-release.yaml/badge.svg)](https://github.com/alisterd51/gratOS/actions/workflows/nightly-release.yaml)

## Presentation

Kernel from scratch (in rust)

## Run

```bash
make run
```

## TODO

- [x] Mandatory:
  - [x] Install GRUB on an virtual image
  - [x] Write an ASM boot code that handles multiboot header, and use GRUB to init and call main function of the kernel itself
  - [x] Write basic kernel code of the choosen language.
  - [x] Compile it with correct flags, and link it to make it bootable.
  - [x] Once all of those steps above are done, you can write some helpers like kernel types or basic functions (strlen, strcmp, ...)
    - [x] string.h
      - [x] memcpy
      - [x] memset
      - [x] memcmp
  - [x] Your work must not exceed 10 MB.
  - [x] Code the interface between your kernel and the screen.
    - [x] vga text_mode driver
  - [x] Display "42" on the screen.
    - [x] clean screen
    - [x] print "42"
  - [x] Makefile:
    Your makefile must compile all your source files with the right flags and the right compiler. Keep in mind that your kernel will use at least two different languages (ASM and whatever-you-choose), so make (<- joke) your Makefile’s rules correctly
- [ ] Bonus:
  - [ ] Add scroll and cursor support to your I/O interface.
  - [ ] Add colors support to your I/O interface.
  - [x] Add helpers like printf / printk in order to print information / debug easily
    - [x] print!
    - [x] println!
  - [x] Handle keyboard entries and print them.
  - [ ] Handle different screens, and keyboard shortcuts to switch easily between then.
- [x] Other:
  - [x] add CI to build and serve an iso of gratOS
