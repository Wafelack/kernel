CC=i686-elf-gcc
CFLAGS=-std=gnu99 -ffreestanding -O2 -Wall -Wextra
BOOTLOADER=src/boot.asm
LINKER=src/linker.ld
BINARY=build/kernel.bin

cfiles=$(wildcard src/*.c)
cobjects=$(patsubst src%,build%,$(patsubst %c,%o,$(wildcard src/*.c)))

run : verify
	qemu-system-i386 -kernel build/kernel.bin
verify : link
	grub-file --is-x86-multiboot $(BINARY)
link : kernel bootloader
	$(CC) -T $(LINKER) -o $(BINARY) -ffreestanding -O2 -nostdlib build/boot.o $(cobjects) -lgcc

kernel : $(cfiles)
	$(CC) -c $(cfiles) -o $(cobjects) $(CFLAGS)

bootloader : $(BOOTLOADER)
	nasm -felf32 src/boot.asm -o build/boot.o

clean:
	rm build/*.o

mrproper:
	rm build/*