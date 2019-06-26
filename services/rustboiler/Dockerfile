# select bin image builder
FROM rustlang/rust:nightly as builder

# create a new empty project for cache persistence
WORKDIR /app/src
RUN USER=root cargo new --bin rustboiler

# copy over the manifests
COPY ./Cargo.toml ./rustboiler/

WORKDIR /app/src/rustboiler

# chache dependencies
RUN cargo build --release

# copy source tree
COPY ./ ./

# build for release
RUN cargo build --release

# pipeline to final image
FROM debian:jessie-slim

EXPOSE 8000
COPY --from=builder /app/src/rustboiler/target/release/rustboiler ./
CMD ["./rustboiler"]