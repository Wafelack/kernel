TARGET := build/kernel.img
FILES := $(shell find src/ -type f)
MEMORY ?= 256M
QEMU_FLAGS :=                      	 \
	--enable-kvm                     \
	-serial mon:stdio                \
	-no-reboot                       \
	-nographic                       \
	-drive file=$(TARGET),format=raw \
	-m $(MEMORY)

$(TARGET): $(FILES)
	cargo xbuild
	dd if=/dev/zero bs=1M count=0 seek=64 of=$(TARGET)
	parted -s $(TARGET) mklabel msdos
	parted -s $(TARGET) mkpart primary 1 100%
	parted -s $(TARGET) set 1 boot on

	echfs-utils -m -p0 $(TARGET) quick-format 32768
	echfs-utils -m -p0 $(TARGET) import limine/limine.sys limine.sys
	echfs-utils -m -p0 $(TARGET) import limine.cfg limine.cfg
	echfs-utils -m -p0 $(TARGET) import target/x86_64/debug/kernel kernel.elf

	./limine/limine-install-linux-x86_64 $(TARGET)
	chmod 666 $(TARGET)
run: clean $(TARGET)
	@qemu-system-x86_64 $(QEMU_FLAGS)
clean:
	cargo clean
	rm build/* -f
.PHONY: clean
