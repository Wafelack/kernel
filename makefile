SRCS := $(shell find ./src -type f -name '*.c')
OBJS := $(subst src/,build/,$(SRCS:.c=.o))
KERNEL_DISK := disk.hdd
CC=x86_64-elf-gcc
CFLAGS :=	       		 \
	-Isrc          		 \
	-std=c11       		 \
	-ffreestanding       \
	-fno-stack-protector \
	-fno-pic 			 \
	-no-pie				 \
	-O1					 \
	-g				     \
	-m64                 \
	-masm=intel			 \
	-mno-red-zone	     \
	-mno-sse			 \
	-mno-avx
LD_FLAGS := 			 	\
	-nostdlib			 	\
	-Tsrc/link.ld		    \
	-z max-page-size=0x1000

$(KERNEL_DISK): kernel.elf
	@echo $(OBJS)
	rm -f $(KERNEL_DISK)
	dd if=/dev/zero bs=8M count=0 of=$(KERNEL_DISK)
	parted -s $(KERNEL_DISK) mklabel msdos
	parted -s $(KERNEL_DISK) mkpart primary 1 100%
	echfs-utils -g -p0 $(KERNEL_DISK) quick-format 4096
	echfs-utils -g -p0 $(KERNEL_DISK) import kernel.elf kernel.elf
	echfs-utils -g -p0 $(KERNEL_DISK) import limine.cfg limine.cfg
	echfs-utils -m -p0 $(KERNEL_DISK) import ./limine/limine.sys limine.sys
	./limine/limine-install-linux-x86_64 $(KERNEL_DISK)
run: $(KERNEL_DISK)
	qemu-system-x86_64 -m 512M -s -device pvpanic -serial stdio -enable-kvm -d cpu_reset -hda ./disk.hdd
build/%.o: $(SRCS)
	$(CC) $(CFLAGS) -c $< -o $@
kernel.elf: $(OBJS)
	$(LD) $(LD_FLAGS) $(OBJS) -o $@
.PHONY: clean
clean:
	rm -rf build/*.o
