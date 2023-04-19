BUILD_DIR  := build

.PHONY: all
all: none.asm
	bash -c "cp -r build/*.elf ../initfs/"

%.elf: %.asm
	nasm -f elf64 -o $(BUILD_DIR)/$@ $<

$(BUILD_DIR):
	mkdir -p $(BUILD_DIR)
