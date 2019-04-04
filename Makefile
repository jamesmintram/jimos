######################################

QEMU_BIN = qemu-system-aarch64
QEMU_SHARED_FLAGS = -M raspi3  -semihosting -serial stdio

KERNEL_DIR  = kernel
KERNEL = $(KERNEL_DIR)/build/blinky

LOAD_ELF = -device loader,file=csrc/build/prog1.elf,addr=0x3F000000,force-raw=true
LOAD_KERNEL = -device loader,file=$(KERNEL).bin,addr=0x0

TIME_STAMP := `/bin/date "+%Y-%m-%d_%H-%M-%S"`

# Setup a build using a docker container
#
# docker-compose build run make all
#


build:
	cd kernel && make all

rund: build
	qemu-system-aarch64 -s -S -d int $(QEMU_SHARED_FLAGS) $(LOAD_KERNEL) $(LOAD_ELF)

runi: build 
	qemu-system-aarch64 -d int $(QEMU_SHARED_FLAGS) $(LOAD_KERNEL) $(LOAD_ELF)

run: build 
	qemu-system-aarch64 $(QEMU_SHARED_FLAGS) $(LOAD_KERNEL) $(LOAD_ELF) 


run_dump: build
	mkdir -p dumps
	qemu-system-aarch64 $(QEMU_SHARED_FLAGS) $(LOAD_KERNEL) $(LOAD_ELF)  &> dumps/kernel_run_$(TIME_STAMP).dump
	