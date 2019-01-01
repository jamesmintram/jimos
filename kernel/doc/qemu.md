To build

./configure --target-list=aarch64-softmmu --enable-modules --enable-tcg-interpreter --enable-debug-tcg --python=/usr/bin/python2.7 --enable-gtk --enable-sdl

make -j12

sudo make install


To run

qemu-system-aarch64 -M raspi3 -semihosting -serial stdio -device loader,file=kernel8.img,addr=0x0   

-display gtk    :: framebuffer
-d int          ::  display interrupts
-s -S           ::  debugging



Loading ram files
    -device loader,file=build/blinky.bin,addr=0x0 

Loading in a raw image
    -device loader,file=../csrc/build/prog1.bin,addr=0x30000000

Loading in an executable image as RAW (Should always use)
    -device loader,file=../csrc/build/prog1.elf,addr=0x30000000,force-raw=true