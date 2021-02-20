CC=i686-elf-gcc
CFLAGS=-std=gnu99 -ffreestanding -O2 -Wall -Wextra -g
BOOTLOADER=src/boot.asm
LINKER=src/linker.ld
BINARY=build/kernel.bin
SRC=src/
DRIVERS=$(SRC)/drivers
TABLES=$(SRC)/tables

BUILD=build/

cfiles=$(SRC)/main.c $(DRIVERS)/vga.c $(TABLES)/gdt.c
cobjects=$(BUILD)/main.o $(BUILD)/vga.o $(BUILD)/gdt.o

run : verify
	qemu-system-i386 -kernel $(BINARY)

verify : link
	grub-file --is-x86-multiboot $(BINARY)
	
link : $(cobjects) bootloader
	$(CC) -T $(LINKER) -o $(BINARY) -ffreestanding -O2 -nostdlib build/boot.o $(cobjects) -lgcc

$(cobjects) : $(cfiles)
	$(CC) -c $(SRC)/main.c -o $(BUILD)/main.o $(CFLAGS)
	$(CC) -c $(DRIVERS)/vga.c -o $(BUILD)/vga.o $(CFLAGS)
	$(CC) -c $(TABLES)/gdt.c -o $(BUILD)/gdt.o $(CFLAGS) 

bootloader : $(BOOTLOADER)
	nasm -felf32 src/boot.asm -o build/boot.o

clean:
	rm build/*.o

mrproper:
	rm build/*