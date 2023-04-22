BUILD_DIR  := build

.PHONY: all
all: none.asm
	bash -c "cp -r build/*.elf ../initfs/"

$(BUILD_DIR)/%.elf: %.asm $(BUILD_DIR)
	nasm -f elf64 -o $@ $<

%.elf: %.c
	clang 

$(BUILD_DIR):
	mkdir -p $(BUILD_DIR)
