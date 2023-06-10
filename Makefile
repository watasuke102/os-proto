BUILD_DIR  := build
MOUNT_DIR  ?= "$(BUILD_DIR)/mnt"

INITFS_ITEM := $(shell find initfs)
COMMON_SRC  := $(shell find common -name "*.rs")
LOADER_SRC  := $(COMMON_SRC) $(shell find loader -name "*.rs")
KERNEL_SRC  := $(COMMON_SRC) $(shell find kernel -name "*.rs")
APPS        := none fib

.PHONY: all r mount umount kill apps loader kernel

all: r
b: $(BUILD_DIR)/image.img
r: $(BUILD_DIR)/image.img
	@echo -e "\e[42m>> Starting... \e[m"
	@qemu-system-x86_64 -s -nographic -m 1G \
		-drive if=pflash,format=raw,readonly=on,file=OVMF_CODE.fd \
		-drive if=pflash,format=raw,file=OVMF_VARS.fd \
		-drive format=raw,file=$(BUILD_DIR)/image.img \

# -d int,cpu_reset
# -device nec-usb-xhci,id=xhci -device usb-kbd -device usb-mouse \

mount:
	sudo mount -o loop $(BUILD_DIR)/image.img $(MOUNT_DIR)
mount-initfs:
	sudo mount -o loop $(BUILD_DIR)/initfs.img $(MOUNT_DIR)
umount:
	sudo umount $(BUILD_DIR)/mnt
kill:
	killall qemu-system-x86_64 -s SIGKILL

apps:
	cd apps && cargo build
	cp -v $(addprefix apps/target/x86_64-unknown-none/debug/, $(APPS)) initfs/

loader: $(BUILD_DIR) $(BUILD_DIR)/loader.efi
$(BUILD_DIR)/loader.efi: $(LOADER_SRC)
	cd loader && cargo build
	cp loader/target/x86_64-unknown-uefi/debug/loader.efi $(BUILD_DIR)/loader.efi

kernel: $(BUILD_DIR) $(BUILD_DIR)/kernel.elf
$(BUILD_DIR)/kernel.elf: $(KERNEL_SRC)
	cd kernel && cargo build
	cp kernel/target/x86_64-unknown-os/debug/kernel $(BUILD_DIR)/kernel.elf

$(BUILD_DIR)/image.img: $(BUILD_DIR) $(BUILD_DIR)/kernel.elf $(BUILD_DIR)/loader.efi $(BUILD_DIR)/initfs.img
	qemu-img create -f raw $@ 128M
	mkfs.fat $@
	mmd   -i $@ EFI
	mmd   -i $@ EFI/BOOT
	mcopy -i $@ $(BUILD_DIR)/loader.efi ::/EFI/BOOT/BOOTX64.EFI
	mcopy -i $@ $(BUILD_DIR)/kernel.elf ::
	mcopy -i $@ $(BUILD_DIR)/initfs.img ::

$(BUILD_DIR)/initfs.img: $(BUILD_DIR) $(INITFS_ITEM) apps
$(BUILD_DIR)/initfs.img: $(BUILD_DIR) $(MOUNT_DIR) $(INITFS_ITEM) apps
	qemu-img create -f raw $@ 8M
	mkfs.fat -n 'INITFS' -s2 -f2 -R32 -F32 $@
	mcopy -i $@ initfs/* ::
