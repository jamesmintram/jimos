Get running:

FreeBSD
    sudo pkg install aarch64-none-elf-gcc

Linux
    Use the gcc toolchain on Jupiter

All
    install rustup
    cargo install xargo
    rustup toolchain install nightly-2019-12-12
    rustup default nightly-2019-12-12
    rustup component add rust-src


FreeBSD
    gmake

Linux
    make