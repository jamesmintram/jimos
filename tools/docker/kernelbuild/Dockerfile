FROM debian:stretch-slim

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH \
    RUST_VERSION=1.33.0

RUN set -eux; \
    apt-get update; \
    apt-get install -y --no-install-recommends \
        ca-certificates \
        gcc \
        libc6-dev \
        libc6-dev-i386 \
        wget \
        make \
        gcc-aarch64-linux-gnu \
        ; \
    apt-get remove -y --auto-remove \
        ; \
    rm -rf /var/lib/apt/lists/*;

RUN useradd -ms /bin/bash rustuser; \
    mkdir -p /opt/rust;\   
    chown rustuser:rustuser /opt/rust

USER rustuser

ENV RUSTUP_HOME=/opt/rust
ENV CARGO_HOME=/opt/rust

RUN cd $RUSTUP_HOME; \
    rustArch='x86_64-unknown-linux-gnu'; \
    url="https://static.rust-lang.org/rustup/archive/1.16.0/${rustArch}/rustup-init"; \
    wget "$url"; \
    # echo "${rustupSha256} *rustup-init" | sha256sum -c -; \
    chmod +x rustup-init; \
    ./rustup-init -y --no-modify-path --default-toolchain $RUST_VERSION; \
    rm rustup-init; \
    chmod -R a+w $RUSTUP_HOME $CARGO_HOME; \
    cat $RUSTUP_HOME/env; 
    # source $RUSTUP_HOME/env; \

ENV PATH="/opt/rust/bin:$PATH"

RUN rustup --version; \
    cargo --version; \
    rustc --version;

RUN rustup install nightly-2019-03-22; \
    rustup override set nightly-2019-03-22; \
    rustup component add rust-src; \
    cargo install xargo;

COPY --chown=rustuser:rustuser test_gcc /test_gcc
COPY --chown=rustuser:rustuser prime_xargo /prime_xargo

RUN cd /test_gcc/ && make; \
    cd /prime_xargo/ && make;
