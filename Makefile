CC=gcc
CFLAGS=-m32 -std=gnu99 -ffreestanding -fno-stack-protector -I "./includes/" -g -Wall -Wextra -Werror  -nostdlib
ASMFLAGS=-f elf32 -F dwarf
LDFLAGS=-I "./includes/" -m elf_i386

OBJECTS=build
COBJECTS=$(patsubst src%, build%, $(patsubst %.c, %.o, $(wildcard src/*.c)))

BOOTLOADER=./boot.asm
CORE=build/boot.o
LINKER=./linker.ld
BINARY=./build/kernel.bin

all : verify
	qemu-system-i386 -kernel $(BINARY)

debug : verify
	qemu-system-i386 -S -gdb tcp::9000 -kernel $(BINARY) -no-reboot -monitor stdio -display sdl -vga std

verify : link
	grub-file --is-x86-multiboot $(BINARY)

link : $(COBJECTS) $(CORE)
	ld -T $(LINKER) -o $(BINARY) $(CORE) $(COBJECTS) $(LDFLAGS)

$(OBJECTS)/%.o : src/%.c _recompile
	$(CC) -c $< -o $@ $(CFLAGS)

$(OBJECTS)/%.o : $(BOOTLOADER) _recompile
	nasm $(BOOTLOADER) -o $(OBJECTS)/boot.o $(ASMFLAGS)

mrproper:
	rm build/*

_recompile:
	echo $(COBJECTS)
