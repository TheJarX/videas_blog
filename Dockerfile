FROM rust:1.81 AS builder

WORKDIR /usr/src/app

# Install cross-compilation tools and dependencies
RUN apt-get update && apt-get install -y \
    gcc-aarch64-linux-gnu \
    g++-aarch64-linux-gnu \
    libc6-dev-arm64-cross \
    libpq-dev

# Set up for cross-compilation
ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc \
    CC_aarch64_unknown_linux_gnu=aarch64-linux-gnu-gcc \
    CXX_aarch64_unknown_linux_gnu=aarch64-linux-gnu-g++

# Install diesel_cli
RUN cargo install diesel_cli --no-default-features --features postgres

COPY Cargo.toml Cargo.lock ./

# Add the target
RUN rustup target add aarch64-unknown-linux-gnu

COPY . .

RUN cargo build --release --target aarch64-unknown-linux-gnu

# Use Ubuntu 22.04 as the base image for the final stage
FROM arm64v8/ubuntu:22.04

RUN apt-get update && apt-get install -y libpq5 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/app/target/aarch64-unknown-linux-gnu/release/videas_blog /usr/local/bin/videas_blog
COPY --from=builder /usr/src/app/wwwroot ./wwwroot
COPY --from=builder /usr/src/app/templates ./templates
COPY --from=builder /usr/src/app/diesel.toml ./diesel.toml
COPY --from=builder /usr/src/app/blog_config.toml ./blog_config.toml

EXPOSE 8080

# Set the startup command
CMD ["videas_blog"]