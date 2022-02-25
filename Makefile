BUILD_DIR := build

.PHONY: run mount umount loader
run: $(BUILD_DIR)/loader.efi
	./boot.sh
mount:
	sudo mount -o loop $(BUILD_DIR)/disk.img $(BUILD_DIR)/mnt
umount:
	sudo umount $(BUILD_DIR)/mnt

loader: $(BUILD_DIR)/loader.efi

$(BUILD_DIR)/loader.efi: loader/src/main.rs
	cd loader && cargo build
	cp loader/target/x86_64-unknown-uefi/debug/loader.efi $(BUILD_DIR)

$(BUILD_DIR)/kernel.elf: kernel/src/main.rs
	cd kernel && cargo build
	cp kernel/target/x86_64-unknown-none-elf/debug/loader.efi $(BUILD_DIR)
