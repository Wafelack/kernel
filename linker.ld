OUTPUT_FORMAT(elf32-i386)
ENTRY(_start) /* Entry point. */

SECTIONS
{
    . = 1M; /* Initialize sections at 1MiB. */

    /* Multiboot header. */
    .text BLOCK(4K) : ALIGN(4K)
    {
        *(.multiboot)
        *(.text)
    }

    /* Read only data. */
    .rodata BLOCK(4K) : ALIGN(4K)
    {
        *(.rodata)
    }

    /* Read & write initialized data. */
    .data BLOCK(4K) : ALIGN(4K)
    {
        *(.data)
    }

    /* Read & write uninitialized data & stack. */
    .bss BLOCK(4K) : ALIGN(4K)
    {
        *(COMMON)
        *(.bss)
    }
}