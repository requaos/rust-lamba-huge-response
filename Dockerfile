FROM lambci/lambda:build-provided

# Install rust
ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH \
    RUST_VERSION=1.62.1 \
    ZIG_VERSION=0.9.1

RUN yum install -y gcc gcc-c++ openssl-devel; \
    curl https://sh.rustup.rs -sSf | sh -s -- --no-modify-path --profile minimal --default-toolchain $RUST_VERSION -y; \
    chmod -R a+w $RUSTUP_HOME $CARGO_HOME; \
    cargo install cargo-lambda; \
    rustup --version; \
    cargo --version; \
    rustc --version;

# Install Zig
RUN curl -L "https://ziglang.org/download/${ZIG_VERSION}/zig-linux-$(uname -m)-${ZIG_VERSION}.tar.xz" | tar -J -x -C /usr/local && \
    ln -s "/usr/local/zig-linux-$(uname -m)-${ZIG_VERSION}/zig" /usr/local/bin/zig

WORKDIR /build/bootstrap

COPY . .

RUN cargo lambda build --release