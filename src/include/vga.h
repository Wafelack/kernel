#ifndef _VGA_H_
#define _VGA_H_

#include <stdint.h>
#include <stddef.h>

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
} Writer;

// Color functions
uint8_t 
make_color(Color fg, Color bg);
uint8_t 
make_entry(uint8_t color, unsigned char character);

// Writer functiosn
Writer 
writer_init(void);
uint8_t 
make_char(uint8_t color, unsigned char character);
void 
writer_setcolor(Writer* writer, uint8_t color);
void 
_writer_putat(Writer* writer, unsigned char c, uint8_t color, size_t x, size_t y);
void 
_writer_put(Writer *writer, char c);
void
_writer_write(Writer *writer, const char *s, size_t size);
void
writer_writes(Writer *writer, const char *s);

#endif