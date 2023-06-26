IMG_DIR    := img
LOADER_DIR := $(IMG_DIR)/EFI/BOOT
OVMF_DIR   := /usr/share/edk2-ovmf/x64

INITFS_ITEM := $(shell find initfs -type f)
COMMON_SRC  := $(shell find common -name "*.rs")
LOADER_SRC  := $(COMMON_SRC) $(shell find loader -name "*.rs")
KERNEL_SRC  := $(COMMON_SRC) $(shell find kernel -name "*.rs" -or -name "*.asm")
APPS        := none fib loop

.PHONY: all b r kill apps loader kernel initfs

all: r
b: loader kernel initfs
r: loader kernel initfs OVMF_VARS.fd
	@echo -e "\e[32;7m>> Starting... \e[m"
	@qemu-system-x86_64 -s -nographic -m 2G \
		-drive if=pflash,format=raw,readonly=on,file=$(OVMF_DIR)/OVMF_CODE.fd \
		-drive if=pflash,format=raw,file=OVMF_VARS.fd \
		-drive format=raw,file=fat:rw:$(IMG_DIR)

OVMF_VARS.fd:
	cp $(OVMF_DIR)/OVMF_VARS.fd .

# -d int,cpu_reset

kill:
	killall qemu-system-x86_64 -s SIGKILL

apps:
	@echo -e "\e[34;1m> Apps\e[m"
	cd apps && cargo build
	cp -v $(addprefix apps/target/x86_64-unknown-none/debug/, $(APPS)) initfs/

loader: $(LOADER_DIR) $(LOADER_DIR)/BOOTX64.EFI
$(LOADER_DIR)/BOOTX64.EFI: $(LOADER_SRC)
	@echo -e "\e[34;1m> loader\e[m"
	cd loader && cargo build
	cp loader/target/x86_64-unknown-uefi/debug/loader.efi $@

kernel: $(IMG_DIR) $(IMG_DIR)/kernel.elf
$(IMG_DIR)/kernel.elf: $(KERNEL_SRC)
	@echo -e "\e[34;1m> kernel\e[m"
	cd kernel && cargo build
	cp kernel/target/x86_64-unknown-os/debug/kernel $@

initfs: $(IMG_DIR) $(IMG_DIR)/initfs.img
$(IMG_DIR)/initfs.img: $(INITFS_ITEM) apps
	@echo -e "\e[34;1m> initfs.img\e[m"
	qemu-img create -f raw $@ 8M
	mkfs.fat -n 'INITFS' -s2 -f2 -R32 -F32 $@
	mcopy -i $@ initfs/* ::

$(IMG_DIR):
	mkdir -p $@
$(LOADER_DIR):
	mkdir -p $@
