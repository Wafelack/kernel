#include "drivers/vga.h"
#include "utils/utils.h"

uint8_t 
make_color(Color fg, Color bg)
{
    return fg | bg << 4;
}

uint16_t 
make_char(uint8_t color, unsigned char character)
{
    return (uint16_t)character | (uint16_t) color << 8;
}

void
clear_screen(WRITER *writer) {
    for (uint32_t i = 0; i < WIDTH * HEIGHT; i++) {
        writer->buffer[i] = make_char(writer->color, ' ');
    }
    writer->column = 0;
    writer->row = 0;
}

void 
writer_setcolor(WRITER* writer, uint8_t color) 
{
    writer->color = color;
}

#ifdef SCROLL
static void
scroll(WRITER *ptr)
{
    memmove(ptr->buffer, ptr->buffer + WIDTH, WIDTH * HEIGHT - WIDTH); // Moves the buffer «one line up».
    memset(ptr->buffer, make_char(ptr->color, ' '), WIDTH); // Fills the created blank line.
    ptr->row--;
}
#endif

void
vga_write(WRITER* writer, const char *word)
{
    uint32_t i = 0;
    while (word[i] != '\0')
    {
        if (word[i] == '\n') {
            writer->row++;
            writer->column = 0;
            i++;
            continue;
        }

        writer->buffer[writer->row * WIDTH + writer->column++] = make_char(writer->color, word[i++]);

        if (writer->column >= WIDTH)
        {
            writer->row++;
            writer->column = 0;
        }

        if (writer->row >= HEIGHT) {
            #ifdef SCROLL
                scroll(writer);
            #else
                clear_screen(writer);
            #endif
        }
        

    }
}

WRITER writer = {
    0,
    0,
    LIGHT_GRAY | BLACK << 4,
    (uint16_t*)BUFFER_ADDR
};
