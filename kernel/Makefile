TARGET ?= aarch64-none-elf
# aarch64-none-elf
CROSS ?= $(TARGET)

DEBUG := 0

CC := $(CROSS)-gcc
OBJDUMP := $(CROSS)-objdump
XARGO := CARGO_INCREMENTAL=0 RUST_TARGET_PATH="$(shell pwd)" xargo

LD_LAYOUT := src/arch/aarch64/layout.ld

RUST_BINARY := $(shell cat Cargo.toml | grep name | cut -d\" -f 2 | tr - _)
RUST_BUILD_DIR := target/$(TARGET)
RUST_DEBUG_LIB := $(RUST_BUILD_DIR)/debug/lib$(RUST_BINARY).a
RUST_RELEASE_LIB := $(RUST_BUILD_DIR)/release/lib$(RUST_BINARY).a

RUST_DEPS = Xargo.toml Cargo.toml build.rs $(LD_LAYOUT) src/*
# EXT_DEPS = $(BUILD_DIR)/crt0.o $(BUILD_DIR)/exceptions.o $(BUILD_DIR)/head.o
EXT_DEPS = $(BUILD_DIR)/head.o $(BUILD_DIR)/crt0.o $(BUILD_DIR)/exceptions.o $(BUILD_DIR)/jtag.o

BUILD_DIR := build
KERNEL := $(BUILD_DIR)/$(RUST_BINARY)
RUST_LIB := $(BUILD_DIR)/$(RUST_BINARY).a

.PHONY:

VPATH = arch/aarch64

all: $(BUILD_DIR)/kernel8.img  $(KERNEL).bin $(KERNEL).dump $(KERNEL).syms

head: $(BUILD_DIR)/head.o $(BUILD_DIR)/exceptions.o $(BUILD_DIR)/crt0.o

check:
	@$(XARGO) check --target=$(TARGET)

$(RUST_DEBUG_LIB): $(RUST_DEPS)
	@echo "+ Building $@ [xargo]"
	@$(XARGO) build --target=$(TARGET) --features "local_crates"

$(RUST_RELEASE_LIB): $(RUST_DEPS)
	@echo "+ Building $@ [xargo --release]"
	@$(XARGO) build --release --target=$(TARGET) --features "local_crates"

ifeq ($(DEBUG),1)
$(RUST_LIB): $(RUST_DEBUG_LIB) | $(BUILD_DIR)
	@cp $< $@
else
$(RUST_LIB): $(RUST_RELEASE_LIB) | $(BUILD_DIR)
	@cp $< $@
endif

$(BUILD_DIR):
	@mkdir -p $@

$(BUILD_DIR)/%.o: %.c | $(BUILD_DIR)
	@echo "+ Building $@ [cc $<]"
	@$(CC) $(CCFLAGS) -c $< -o $@

#
#	TODO: Fix this up
######################################

$(BUILD_DIR)/head.o: src/arch/aarch64/head.S | $(BUILD_DIR)
	@echo "+ Building $@ [cc $<]"
	@$(CC) $(CCFLAGS) -c $< -o $@

$(BUILD_DIR)/crt0.o: src/arch/aarch64/crt0.S | $(BUILD_DIR)
	@echo "+ Building $@ [cc $<]"
	@$(CC) $(CCFLAGS) -c $< -o $@

$(BUILD_DIR)/exceptions.o: src/arch/aarch64/exceptions.S | $(BUILD_DIR)
	@echo "+ Building $@ [cc $<]"
	@$(CC) $(CCFLAGS) -c $< -o $@

$(BUILD_DIR)/jtag.o: src/arch/aarch64/jtag.c | $(BUILD_DIR)
	@echo "+ Building $@ [cc $<]"
	@$(CC) $(CCFLAGS) -c $< -o $@

######################################


$(KERNEL).elf: $(EXT_DEPS) $(RUST_LIB) | $(BUILD_DIR)
	@echo "+ Building $@ [ld $^]"
	@$(CROSS)-ld  --gc-sections -o $@ $^ -T$(LD_LAYOUT)


$(KERNEL).syms: $(KERNEL).elf | $(BUILD_DIR)
	@echo "+ Generating symbols files"
	@$(CROSS)-objdump $< --syms > $@

$(KERNEL).dump: $(KERNEL).elf | $(BUILD_DIR)
	@echo "+ Dumping $@ [objdump $<]"
	@$(CROSS)-objdump $< -d > $@

$(KERNEL).bin: $(KERNEL).elf | $(BUILD_DIR)
	@echo "+ Building $@ [objcopy $<]"
	@$(CROSS)-objcopy $< -O binary $@

$(BUILD_DIR)/kernel8.img: $(KERNEL).bin
	cp $< $@

clean:
	$(XARGO) clean
	rm -rf $(BUILD_DIR)


######################################

QEMU_BIN = qemu-system-aarch64
QEMU_SHARED_FLAGS = -M raspi3  -semihosting -serial stdio

LOAD_ELF = -device loader,file=../csrc/build/prog1.elf,addr=0x3F000000,force-raw=true
LOAD_KERNEL = -device loader,file=$(KERNEL).bin,addr=0x0

TIME_STAMP := `/bin/date "+%Y-%m-%d_%H-%M-%S"`


rund: all
	qemu-system-aarch64 -s -S -d int $(QEMU_SHARED_FLAGS) $(LOAD_KERNEL) $(LOAD_ELF)

runi: all
	qemu-system-aarch64 -d int $(QEMU_SHARED_FLAGS) $(LOAD_KERNEL) $(LOAD_ELF)

run: all
	qemu-system-aarch64 $(QEMU_SHARED_FLAGS) $(LOAD_KERNEL) $(LOAD_ELF)


run_dump: all
	mkdir -p dumps
	qemu-system-aarch64 $(QEMU_SHARED_FLAGS) $(LOAD_KERNEL) $(LOAD_ELF)  &> dumps/kernel_run_$(TIME_STAMP).dump

gdb: .PHONY
	aarch64-linux-gnu-gdb -x gdb/qemu_attach.gdb