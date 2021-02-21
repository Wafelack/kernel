CC=i686-elf-gcc
CFLAGS=-std=gnu99 -ffreestanding -fno-stack-protector -I "./includes/" -g -Wall -Wextra -Werror
ASMFLAGS=-f elf32 -F dwarf
LDFLAGS=-I "./includes/" -lgcc -nostdlib

OBJECTS=build
COBJECTS=$(patsubst src%, build%, $(patsubst %.c, %.o, $(wildcard src/*.c)))

BOOTLOADER=./assembly/boot.asm
GDT=./assembly/gdt.asm
CORE=build/boot.o
LINKER=./linker.ld
BINARY=./build/kernel.bin

debug : verify
	qemu-system-i386 -S -gdb tcp::9000 -kernel $(BINARY) -no-reboot -monitor stdio -display sdl -vga std

run : verify
	qemu-system-i386 -kernel $(BINARY)

verify : link
	grub-file --is-x86-multiboot $(BINARY)

link : $(COBJECTS) $(CORE)
	$(CC) -T $(LINKER) -o $(BINARY) $(CORE) build/gdt_asm.o $(COBJECTS) $(LDFLAGS)

$(OBJECTS)/%.o : src/%.c _recompile
	$(CC) -c $< -o $@ $(CFLAGS)

$(OBJECTS)/%.o : $(GDT) _recompile
	nasm $(GDT) -o build/gdt_asm.o $(ASMFLAGS)

$(OBJECTS)/%.o : $(BOOTLOADER) _recompile
	nasm $(BOOTLOADER) -o $(CORE) $(ASMFLAGS)

mrproper:
	rm build/*

_recompile:
	echo $(COBJECTS)
