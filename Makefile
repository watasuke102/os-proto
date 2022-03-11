BUILD_DIR  := build
COMMON_SRC := $(wildcard common/src/*.rs)
LOADER_SRC := $(wildcard loader/src/*.rs)
KERNEL_SRC := $(wildcard kernel/src/*.rs)
KERNEL_SRC += $(wildcard kernel/src/memory/*.rs)
KERNEL_SRC += kernel/src/entry.asm

.PHONY: run mount umount kill loader kernel
run: $(BUILD_DIR)/kernel.elf $(BUILD_DIR)/loader.efi
	./boot.sh
mount:
	sudo mount -o loop $(BUILD_DIR)/disk.img $(BUILD_DIR)/mnt
umount:
	sudo umount $(BUILD_DIR)/mnt
kill:
	killall qemu-system-x86_64 -s SIGKILL

loader: $(BUILD_DIR)/loader.efi
kernel: $(BUILD_DIR)/kernel.elf

$(BUILD_DIR)/loader.efi: $(COMMON_SRC) $(LOADER_SRC)
	cd loader && cargo build
	cp loader/target/x86_64-unknown-uefi/debug/loader.efi $(BUILD_DIR)/loader.efi

$(BUILD_DIR)/kernel.elf: $(COMMON_SRC) $(KERNEL_SRC)
	cd kernel && cargo build
	cp kernel/target/x86_64-unknown-os/debug/kernel $(BUILD_DIR)/kernel.elf
