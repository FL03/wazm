FROM rust:latest as builder-base

RUN rustup toolchain add nightly && \
    rustup target add wasm32-unknown-unknown --toolchain nightly && \
    rustup target add wasm32-unknown-unknown --toolchain nightly && \
    cargo install wasm-pack
