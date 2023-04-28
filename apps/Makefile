BUILD_DIR := build
SRC       := $(shell find . -name "*.asm" -or -name "*.c")

.PHONY: all
all: $(addsuffix .elf, $(basename $(SRC)))
	bash -c "cp -rv build/*.elf ../initfs/"

%.elf: %.asm $(BUILD_DIR)
	nasm -f elf64 -o $(BUILD_DIR)/$*.o $<
	ld -o $(BUILD_DIR)/$*.elf $(BUILD_DIR)/$*.o

%.elf: %.c $(BUILD_DIR)
	clang -nostartfiles -fPIC -g -o $(BUILD_DIR)/$@ $<

$(BUILD_DIR):
	mkdir -p $(BUILD_DIR)
