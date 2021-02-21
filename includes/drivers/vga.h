#ifndef _VGA_H_
#define _VGA_H_

#include "../utils/includes.h"

#define BUFFER_ADDR 0xB8000

typedef enum {
    BLACK = 0,
    BLUE = 1,
    GREEN = 2,
    CYAN = 3,
    RED = 4,
    MAGENTA = 5,
    BROWN = 6,
    LIGHT_GRAY = 7,
    DARK_GRAY = 8,
    LIGHT_BLUE = 9,
    LIGHT_GREEN = 10,
    LIGHT_CYAN = 11,
    LIGHT_RED = 12,
    LIGHT_MAGENTA = 13,
    LIGHT_BROWN = 14,
    WHITE = 15,
} Color;

#define WIDTH 80
#define HEIGHT 25

typedef struct {
    size_t row;
    size_t column;
    uint8_t color;
    uint16_t* buffer;
} WRITER;

// Color functions
uint8_t 
make_color(Color fg, Color bg);
uint8_t 
make_entry(uint8_t color, unsigned char character);

// Writer functions
uint16_t 
make_char(uint8_t color, unsigned char character);
void 
writer_setcolor(WRITER* writer, uint8_t color);
void
vga_write(WRITER* writer, const char *word);
void
vga_writeln(WRITER* writer, const char *word);

void
clear_screen(WRITER *writer);

#define kprint(x) vga_write(&writer, x) // Todo: write a printf.

#endif