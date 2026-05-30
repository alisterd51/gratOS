# gratOS

[![Nightly Release](https://github.com/alisterd51/gratOS/actions/workflows/nightly-release.yaml/badge.svg)](https://github.com/alisterd51/gratOS/actions/workflows/nightly-release.yaml)

## Presentation

Kernel from scratch written in [rust](https://www.rust-lang.org/)

## Run

### From source

```bash
git clone https://github.com/alisterd51/gratOS.git
cd gratOS
make run
```

### From nightly release

```bash
wget https://github.com/alisterd51/gratOS/releases/download/nightly/gratos.x86.grub.iso
qemu-system-i386 -cdrom gratos.x86.grub.iso
```

## TODO

- [x] KFS-1 (gratOS:0.1.0):
  - [x] Mandatory:
    - [x] Install GRUB on an virtual image
    - [x] Write an ASM boot code that handles multiboot header, and use GRUB to init and call main function of the kernel itself
    - [x] Write basic kernel code of the choosen language.
    - [x] Compile it with correct flags, and link it to make it bootable.
    - [x] Once all of those steps above are done, you can write some helpers like kernel types or basic functions (strlen, strcmp, ...)
      - [x] string.h
        - [x] memcpy
        - [x] memmove
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
  - [x] Bonus:
    - [x] Add scroll and cursor support to your I/O interface.
      - [x] scroll
      - [x] cursor
      - [x] history
      - [x] interprete escape sequences such as `CSI n A`
      - [x] add consts such as: `CURSOR_UP = "\x1B[A"`
    - [x] Add colors support to your I/O interface.
      - [x] interpret color escape sequences such as `CSI n m`
      - [x] add consts such as: `FG_RED = "\x1b[31m"`
      - [x] add default color
      - [x] add tests in debug mode
    - [x] Add helpers like printf / printk in order to print information / debug easily
      - [x] print!
      - [x] println!
    - [x] Handle keyboard entries and print them.
    - [x] Handle different screens, and keyboard shortcuts to switch easily between then.
- [x] KFS-2 (gratOS:0.2.0):
  - [x] Mandatory:
    - [x] create a Global Descriptor Table
    - [x] Your GDT must contain:
      - [x] Kernel Code
      - [x] Kernel Data
      - [x] Kernel stack
      - [x] User code
      - [x] User data
      - [x] User stack
    - [x] Your work must not exceed 10 MB.
    - [x] You must declare your GDT to the BIOS.
    - [x] The GDT must be set at address 0x00000800.
    - [x] When this is done, you have to code a tool to print the kernel stack, in a human-friendly way. (Tip: If you haven’t made a printk yet, now is a good time !)
  - [x] Bonus:
    - [x] Assuming your keyboard work correctly in your Kernel, and you able to catch an entry, let’s code a Shell !
        Not a POSIX Shell, just a minimalistic shell with a few commands, for debugging purposes.
    - [x] For example, you could implement the print-kernel-stack-thing in this shell, and some other things like reboot, halt and such.
      - [x] hello
      - [x] print-kernel-stack-thing
      - [x] reboot
      - [x] halt
- [x] KFS-3 (gratOS:0.3.0):
  - [x] Mandatory:
    - [x] You must enable memory paging in your Kernel
    - [x] You must code a memory structure that handle paging and memory rights (Careful, you don’t have the tools yet to know who’s accessing the memory, so all of this is theoric at the moment)
    - [x] You must define Kernel and User space
    - [x] You must implement a function to create / get memory pages
    - [x] You must implement functions to `allocate`, `free` and `get` size of a variable
    - [x] You must implement those functions for `virtual` and `physical` memory
    - [x] You must handle "kernel panics" (Print, stop the kernel)
    - [x] A complete memory code structure, with pagination handling
    - [x] Read and Write rights on memory
    - [x] User space memory and Kernel space memory
    - [x] Physical and Virtual memory
    - [x] Code helpers for physical memory:
      - [x] kmalloc (dma_alloc)
      - [x] kfree (dma_free)
      - [ ] ksize (irrelevant in rust)
      - [ ] kbrk (irrelevant in rust)
    - [x] Code helpers for virtual memory: (GlobalAlloc)
      - [x] vmalloc (alloc)
      - [x] vfree (dealloc)
      - [ ] vsize (irrelevant in rust)
      - [x] vbrk (sbrk)
    - [x] Kernel Panic handling
    - [x] Your work should not exceed 10 MB
  - [x] Bonus:
    - [x] try to implement `memory dumping` and `debug` in the last "mini-shell" subject
- [ ] KFS-4 (gratOS:0.4.0):
  - [ ] Mandatory:
    - [ ] Registers cleaning
    - [ ] Stack saving
    - [ ] Hardware Interrupts
    - [ ] Software Interrupts
    - [ ] A Interrupts Descriptor Table
    - [ ] Sigal handling and scheduling
    - [ ] Global Panic Fault handling
    - [ ] Create an Interrupts Descriptor Table, fill it and register it
    - [ ] A signal-callback system on your Kernel API
    - [ ] An interface to schedule signals
    - [ ] An interface to clean registers before a panic / halt
    - [ ] An interface to save the stack before a panic
    - [ ] implement a IDT keyboard handling system
  - [ ] Bonus:
    - [ ] It has not been said, but syscalls are also handled by the IDT. You can’t implement them now (No processus / Execution), but a good start could be coding the base functions for it, it could save you some work.
    - [ ] Also, you can add some features to the keyboard handler, for example multi layouts (qwerty, azerty), base functions like get_line (just like read: waits for characters and return them when \n is pressed).

- [x] Other:
  - [x] add CI to build and serve an iso of gratOS
  - [x] add multiboot/multiboot2 helpers
