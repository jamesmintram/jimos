make clean && make
qemu-system-aarch64 -M raspi3 -semihosting -serial stdio -device loader,file=build/blinky.bin,addr=0x0  -device loader,file=../csrc/build/prog1.elf,addr=0x3F000000,force-raw=true