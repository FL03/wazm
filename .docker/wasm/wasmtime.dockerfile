FROM debian:slim as builder-base

RUN sudo apt-get update -y && sudo apt-get upgrade -y

RUN sudo apt-get install -y && \
    curl

FROM builder-base as installer

RUN curl https://wasmtime.dev/install.sh -sSf | bash

FROM installer as final

RUN wasmtime --version

ENTRYPOINT [ "wasmtime" ]