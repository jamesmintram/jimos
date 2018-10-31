To build

./configure --target-list=aarch64-softmmu --enable-modules --enable-tcg-interpreter --enable-debug-tcg --python=/usr/bin/python2.7 --enable-gtk --enable-sdl

make -j12

sudo make install


To run

qemu-system-aarch64 -M raspi3 -semihosting -serial stdio -device loader,file=kernel8.img,addr=0x0   

-display gtk    :: framebuffer
-d int          ::  display interrupts
-s -S           ::  debugging
