BUILD_DIR  := build
MOUNT_DIR  ?= "$(BUILD_DIR)/mnt"
EFI_DIR    ?= "$(MOUNT_DIR)/EFI/BOOT"

INITFS_ITEM := $(shell find initfs)
COMMON_SRC  := $(shell find common -name "*.rs")
LOADER_SRC  := $(COMMON_SRC) $(shell find loader -name "*.rs")
KERNEL_SRC  := $(COMMON_SRC) $(shell find kernel -name "*.rs")
APPS        := none

.PHONY: all r mount umount kill apps loader kernel

all: $(BUILD_DIR)/image.img
	make r

b: loader kernel
r: $(BUILD_DIR)/image.img
	@echo -e "\e[42m>> Starting... \e[m"
	@qemu-system-x86_64 -s -nographic -serial mon:stdio -m 1G \
		-drive if=pflash,format=raw,readonly=on,file=OVMF_CODE.fd \
		-drive if=pflash,format=raw,file=OVMF_VARS.fd \
		-drive format=raw,file=$(BUILD_DIR)/image.img \

# -d int,cpu_reset
# -device nec-usb-xhci,id=xhci -device usb-kbd -device usb-mouse \

mount:
	sudo mount -o loop $(BUILD_DIR)/image.img $(BUILD_DIR)/mnt
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

$(BUILD_DIR)/image.img: $(BUILD_DIR) $(MOUNT_DIR) $(BUILD_DIR)/kernel.elf $(BUILD_DIR)/loader.efi $(BUILD_DIR)/initfs.img
	qemu-img create -f raw $@ 128M
	mkfs.fat $@
	sudo mount -o loop $@ $(MOUNT_DIR)
	[ ! -e $(EFI_DIR) ] && sudo mkdir -p $(EFI_DIR)
	sudo cp $(BUILD_DIR)/loader.efi $(EFI_DIR)/BOOTX64.EFI
	sudo cp $(BUILD_DIR)/kernel.elf $(MOUNT_DIR)
	sudo cp $(BUILD_DIR)/initfs.img $(MOUNT_DIR)
	sudo umount $(MOUNT_DIR)

$(BUILD_DIR)/initfs.img: $(BUILD_DIR) $(MOUNT_DIR) $(INITFS_ITEM) apps
	qemu-img create -f raw $@ 8M
	mkfs.fat -n 'INITFS' -s2 -f2 -R32 -F32 $@
	sudo mount -o loop $@ $(MOUNT_DIR)
	sudo cp initfs/* $(MOUNT_DIR)
	sudo umount $(MOUNT_DIR)

$(BUILD_DIR):
	mkdir -p $(BUILD_DIR)

$(MOUNT_DIR):
	mkdir -p $(MOUNT_DIR)
