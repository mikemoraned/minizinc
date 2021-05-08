FROM liuchong/rustup:stable AS builder

# Make a fake Rust app to keep a cached layer of compiled crates
RUN USER=root cargo new app
WORKDIR /usr/src/app
COPY Cargo.toml Cargo.lock ./
# Needs at least a main.rs file with a main function
RUN mkdir src && echo "fn main(){}" > src/main.rs
# Will build all dependent crates in release mode
RUN cargo build --release

# Copy the rest
COPY . .
# Build (install) the actual binaries
RUN cargo install --path .

RUN echo -n BUILD_TAG= >> /root/.cargo/bin/.env
RUN date >> /root/.cargo/bin/.env
RUN echo >> /root/.cargo/bin/.env
RUN cat /root/.cargo/bin/.env

# Runtime image
FROM debian:stretch

# Run as "app" user
RUN useradd -ms /bin/bash app

USER app
WORKDIR /app

# Get compiled binaries from builder's cargo install directory
COPY --from=builder /root/.cargo/bin/ /app/
COPY --from=builder /root/.cargo/bin/.env /app/.env

RUN ls -al /app/
RUN cat /app/.env

# No CMD or ENTRYPOINT, see fly.toml with `cmd` override.
