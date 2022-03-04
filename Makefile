BUILD_DIR  := build
LOADER_SRC := $(wildcard loader/src/*.rs)
KERNEL_SRC := $(wildcard kernel/src/*.rs)
KERNEL_SRC += $(wildcard kernel/src/memory/*.rs)

.PHONY: run mount umount loader kernel
run: $(BUILD_DIR)/kernel.elf $(BUILD_DIR)/loader.efi
	./boot.sh
mount:
	sudo mount -o loop $(BUILD_DIR)/disk.img $(BUILD_DIR)/mnt
umount:
	sudo umount $(BUILD_DIR)/mnt

loader: $(BUILD_DIR)/loader.efi
kernel: $(BUILD_DIR)/kernel.elf

#$(BUILD_DIR)/loader.efi: ~/program/mikanos-book/src/MikanLoaderPkg/Main.c
#	cp ~/program/mikanos-book/build/Loader.efi build/loader.efi
$(BUILD_DIR)/loader.efi: $(LOADER_SRC)
	cd loader && cargo build
	cp loader/target/x86_64-unknown-uefi/debug/loader.efi $(BUILD_DIR)

$(BUILD_DIR)/kernel.elf: $(KERNEL_SRC)
	cd kernel && cargo build
	cp kernel/target/spec/debug/kernel $(BUILD_DIR)/kernel.elf
