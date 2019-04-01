TARGET ?= aarch64-linux-gnu
CROSS ?= $(TARGET)

CC := $(CROSS)-gcc
OBJDUMP := $(CROSS)-objdump

LD_LAYOUT := layout.ld

BUILD_DIR := build
PROGRAM := $(BUILD_DIR)/prog1

.PHONY: all clean check

VPATH = ext

all: $(PROGRAM).bin  $(PROGRAM).dump $(PROGRAM).syms

$(BUILD_DIR):
	@mkdir -p $@

$(BUILD_DIR)/%.o: %.c | $(BUILD_DIR)
	@echo "+ Building $@ [cc $<]"
	@$(CC) $(CCFLAGS) -c $< -o $@

$(PROGRAM).elf: $(BUILD_DIR)/main.o | $(BUILD_DIR)
	@echo "+ Building $@ [ld $^]"
	@$(CROSS)-ld --just-symbols=../build/blinky.elf -o $@ $^ -T$(LD_LAYOUT)
	# @$(CROSS)-ld --gc-sections --just-symbols=../build/blinky.elf --verbose -e __start -o $@ $^ -T$(LD_LAYOUT)


main.o : main.c  | $(BUILD_DIR)
	@$(CC) $(CCFLAGS) -c main.c -o $(BUILD_DIR)/main.o

$(PROGRAM).hex: $(PROGRAM).elf | $(BUILD_DIR)
	@echo "+ Building $@ [objcopy $<]"
	@$(CROSS)-objcopy $< -O ihex $@

$(PROGRAM).syms: $(PROGRAM).elf | $(BUILD_DIR)
	@echo "+ Generating symbols files"
	@$(CROSS)-objdump $< --syms > $@

$(PROGRAM).dump: $(PROGRAM).elf | $(BUILD_DIR)
	@echo "+ Dumping $@ [objdump $<]"
	@$(CROSS)-objdump $< -d > $@

$(PROGRAM).bin: $(PROGRAM).elf | $(BUILD_DIR)
	@echo "+ Building $@ [objcopy $<]"
	@$(CROSS)-objcopy $< -O binary $@

clean:
	rm -rf $(BUILD_DIR)
