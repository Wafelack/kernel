#include "vga.h"

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

WRITER 
writer_init(void) 
{
    uint8_t color = make_color(LIGHT_GRAY, BLACK);
    uint16_t *buffer = (uint16_t*) 0xB8000;
    WRITER writer = {
        0,
        0,
        color,
        buffer,
    };
    clear_screen(&writer);
    return writer;
}

void 
writer_setcolor(WRITER* writer, uint8_t color) 
{
    writer->color = color;
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
        }

        writer->buffer[writer->row * WIDTH + writer->column++] = make_char(writer->color, word[i++]);

        if (word[i - 1] == '\n') {
            writer->column = 0; // hide \n character
        }

        if (writer->column >= WIDTH)
        {
            writer->column = 0;
        }

        if (writer->row >= HEIGHT) {
            clear_screen(writer); // TODO: Implement scrolling
        }
        

    }
}