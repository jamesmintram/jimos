The idea is to build and test the RPi3 drivers in another OS. FreeBSD seems ideal for that, being simpler + self contained. Also I have the book.

The idea will be to build as much of the driver as possible in Rust - developing an interface to the FreeBSD driver system where required. Should be interesting to see how far I can get when you consider the amount of custom makefile magic involved with FreeBSD driver dev.

Initially it is probably worth developing the drivers in C - as a way to get familiarity with the driver development process and the Rpi3 platform. 

Some of the drivers to build:

- UART
- Framebuffer driver
- USB - patched together from various minimal libs on the Rpi forums
- Network? - 

Is there a device tree blob that is used?
- Can I disable the video/FB driver?
- Can I disable the USB driver?

https://wiki.freebsd.org/action/show/arm/crossbuild?action=show&redirect=FreeBSD%2Farm%2Fcrossbuild

https://github.com/freebsd/crochet