git clone https://github.com/ntfreak/openocd.git
cd openocd
git submodule init
git submodule update
sudo apt-get install libtool pkg-config texinfo libusb-dev libusb-1.0.0-dev libftdi-dev autoconf
autoreconf -i
./configure --prefix=/opt/openocd




`openocd -f interface/jlink.cfg  -f rpi3.cfg`

Useful link
    https://www.suse.com/c/debugging-raspberry-pi-3-with-jtag/