BUILD_DIR  := build
COMMON_SRC := $(wildcard common/src/*.rs)
LOADER_SRC := $(wildcard loader/src/*.rs)
KERNEL_SRC := $(wildcard kernel/src/*.rs)
KERNEL_SRC += $(wildcard kernel/src/memory/*.rs)
KERNEL_SRC += kernel/src/entry.asm

.PHONY: all run mount umount kill loader kernel $(BUILD_DIR)/kernel.elf $(BUILD_DIR)/loader.efi
all: $(BUILD_DIR)/kernel.elf $(BUILD_DIR)/loader.efi run
run:
	./boot.sh
mount:
	sudo mount -o loop $(BUILD_DIR)/disk.img $(BUILD_DIR)/mnt
umount:
	sudo umount $(BUILD_DIR)/mnt
kill:
	killall qemu-system-x86_64 -s SIGKILL

loader: $(BUILD_DIR)/loader.efi
kernel: $(BUILD_DIR)/kernel.elf

$(BUILD_DIR)/loader.efi:
	cd loader && cargo build
	cp loader/target/x86_64-unknown-uefi/debug/loader.efi $(BUILD_DIR)/loader.efi

$(BUILD_DIR)/kernel.elf:
	cd kernel && cargo build
	cp kernel/target/x86_64-unknown-os/debug/kernel $(BUILD_DIR)/kernel.elf
