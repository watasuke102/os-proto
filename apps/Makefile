BUILD_DIR := build
SRC       := $(shell find . -name "*.asm")

.PHONY: all
all: $(SRC:.asm=.elf)
	bash -c "cp -r build/*.elf ../initfs/"

%.elf: %.asm $(BUILD_DIR)
	nasm -f elf64 -o $(BUILD_DIR)/$@ $<

$(BUILD_DIR):
	mkdir -p $(BUILD_DIR)
