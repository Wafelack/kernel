#include <stddef.h>
#include "include/vga.h"

void
kernel_main(void) 
{
    Writer writer = writer_init();
    writer_writes(&writer, "Hello, World !\n");
}