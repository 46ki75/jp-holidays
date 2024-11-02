FROM rust:latest

RUN apt-get update -y \
    && cargo install cargo-binstall \
    && cargo binstall cargo-watch 

COPY .bashrc /root/.bashrc
