BUILD_DIR := build

.PHONY: run mount umount loader
run: $(BUILD_DIR)/kernel.elf $(BUILD_DIR)/loader.efi
	./boot.sh
mount:
	sudo mount -o loop $(BUILD_DIR)/disk.img $(BUILD_DIR)/mnt
umount:
	sudo umount $(BUILD_DIR)/mnt

loader: $(BUILD_DIR)/loader.efi

$(BUILD_DIR)/loader.efi: ~/program/mikanos-book/src/MikanLoaderPkg/Main.c#loader/src/main.rs
	cp ~/program/mikanos-book/build/Loader.efi build/loader.efi
#	cd loader && cargo build
#	cp loader/target/x86_64-unknown-uefi/debug/loader.efi $(BUILD_DIR)

$(BUILD_DIR)/kernel.elf: kernel/src/main.rs
	cd kernel && cargo build
	cp kernel/target/spec/debug/kernel $(BUILD_DIR)/kernel.elf
