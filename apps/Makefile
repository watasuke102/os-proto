BUILD_DIR := build
SRC       := $(shell find . -name "*.asm")

.PHONY: all
all: $(SRC:.asm=.elf)
	bash -c "cp -rv build/*.elf ../initfs/"

%.elf: %.asm $(BUILD_DIR)
	nasm -f elf64 -o $(BUILD_DIR)/$*.o $<
	ld -o $(BUILD_DIR)/$*.elf $(BUILD_DIR)/$*.o

$(BUILD_DIR):
	mkdir -p $(BUILD_DIR)
