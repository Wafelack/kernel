#include "utils/utils.h"

static uint8_t
digit_amount(int32_t number) {
    uint8_t toret = 0;

    do {
        number /= 10;
        ++toret;
    } while (number > 0);
    return toret;
}

void
itoa(char *buffer, int32_t number)
{
    uint8_t digits_amnt = digit_amount(number);
    uint8_t neg = number < 0;
    uint8_t i = 0;

    if (neg) i++;

    number = number < 0 ? -1*number : number;
    if (neg) buffer[0] = '-';

    do {
        if (!neg)
            buffer[digits_amnt - i - 1] = (char)((number % 10) + (char)'0');
        else
            buffer[digits_amnt - i + 1] = (char)((number % 10) + (char)'0');
        number /= 10;
        ++i;
    } while (number != 0);
    for (; i < 7; i++) {
        buffer[i] = '\0';
    }
}

void*
memset(void* dest, int32_t ch, size_t count)
{
    int8_t casted = (int8_t)ch;
    for (size_t i = 0; i < count; i++) {
        ((uint8_t*)dest)[i] = casted;
    }
    return dest;
}

void*
memcpy(void* dest, const void* src, size_t count)
{
    for (size_t i = 0; i < count; i++) {
        ((uint8_t*)dest)[i] = ((uint8_t*)src)[i];
    }
    return dest;
}

void*
memmove(void* dest, const void* src, size_t count)
{
    for (size_t i = 0; i < count; i++)
    {
        ((uint8_t*)dest)[i] = ((uint8_t*)src)[i];
    }
    return dest;
}