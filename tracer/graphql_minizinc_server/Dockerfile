FROM rust:1.54.0-buster AS builder

# Make a fake Rust app to keep a cached layer of compiled crates
RUN USER=root cargo new app
WORKDIR /usr/src/app
COPY Cargo.toml Cargo.lock ./
# Needs at least minimal versions of the things referred to in `Cargo.toml`
RUN mkdir -p src/bin
RUN echo "fn main(){}" > src/bin/server.rs
RUN touch src/lib.rs
# Will build all dependent crates in release mode
RUN cargo build --release

# Copy the rest
COPY . .
# Build (install) the actual binaries
RUN cargo install --path .

# Runtime image
FROM debian:buster

# Run as "app" user
RUN useradd -ms /bin/bash app

USER app
WORKDIR /app

# Get compiled binaries from builder's cargo install directory
COPY --from=builder /usr/local/cargo/bin/ /app/

# No CMD or ENTRYPOINT, see fly.toml with `cmd` override.
