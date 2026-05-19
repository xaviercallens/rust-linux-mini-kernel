/* Rust Linux Mini Kernel - x86_64 Linker Script */

OUTPUT_FORMAT(elf64-x86-64)
OUTPUT_ARCH(i386:x86-64)
ENTRY(_start)

/* Kernel starts at 1 MB (standard for x86_64 kernels) */
KERNEL_BASE = 0x100000;

SECTIONS
{
    . = KERNEL_BASE;

    /* Code section */
    .text ALIGN(4K) : {
        KEEP(*(.multiboot))
        *(.text .text.*)
    }

    /* Read-only data */
    .rodata ALIGN(4K) : {
        *(.rodata .rodata.*)
    }

    /* Initialized data */
    .data ALIGN(4K) : {
        *(.data .data.*)
    }

    /* Uninitialized data (BSS) */
    .bss ALIGN(4K) : {
        __bss_start = .;
        *(.bss .bss.*)
        *(COMMON)
        __bss_end = .;
    }

    /* Discard debug info and other unnecessary sections */
    /DISCARD/ : {
        *(.comment)
        *(.eh_frame)
        *(.note.*)
    }

    /* End of kernel */
    . = ALIGN(4K);
    __kernel_end = .;
}
