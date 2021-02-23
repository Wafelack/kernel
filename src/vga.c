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

static void
scroll(WRITER *ptr)
{
    uint16_t *new = (uint16_t*)BUFFER_ADDR;
    for (size_t i = WIDTH; i < WIDTH * HEIGHT ; i ++)
    {
        new[i - WIDTH] = ptr->buffer[i];
    }
    ptr->buffer = new;
    ptr->row--;
}

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
                scroll(writer);
        }
        

    }
}

WRITER writer = {
    0,
    0,
    LIGHT_GRAY | BLACK << 4,
    (uint16_t*)BUFFER_ADDR
};
