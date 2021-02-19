#include "include/vga.h"

uint8_t 
make_color(Color fg, Color bg)
{
    return fg | bg << 4;
}

uint8_t 
make_char(uint8_t color, unsigned char character)
{
    return (uint16_t)character | (uint16_t) color << 8;
}

Writer 
writer_init(void) 
{
    uint8_t color = make_color(LIGHT_GRAY, BLACK);
    uint16_t *buffer = (uint16_t*) 0xB8000;
    for (size_t y = 0; y < HEIGHT; y++) {
        for (size_t x = 0; x < WIDTH; x++) {
            const size_t i = y * WIDTH + x;
            buffer[i] = make_char(' ', color);
        }
    }
    Writer toret = {
        0,
        0,
        color,
        buffer,
    };
    return toret;
}

void 
writer_setcolor(Writer* writer, uint8_t color) 
{
    writer->color = color;
}

void 
_writer_putat(Writer* writer, unsigned char c, uint8_t color, size_t x, size_t y)
{

    if (c == '\n') {
        ++writer->row;
        writer->column = 0;
    } else if (c == '\r') {
        writer->column = 0;
    }
    const size_t i = y * WIDTH + x;
    writer->buffer[i] = make_char(c, color);
}

void 
_writer_put(Writer *writer, char c)
{
    _writer_putat(writer, c, writer->color, writer->column, writer->row);
    if (++writer->column == WIDTH) {
        writer->column = 0;
        if (++writer->row == HEIGHT) {
            writer->row = 0;
        }
    }
}

void
_writer_write(Writer *writer, const char *s, size_t size)
{
    for (size_t i = 0; i < size; i++) {
        _writer_put(writer, s[i]);
    }
}

static size_t
len(const char *s) {
    size_t i = 0;
    while (s[i] != '\0') {
        i++;
    }
    return i;
}

void
writer_writes(Writer *writer, const char *s) {
    _writer_write(writer, s, len(s));
}