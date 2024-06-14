FROM debian:latest-slim as builder-base

RUN sudo apt-get update -y && \
    sudo apt-get upgrade -y

RUN sudo apt-get install -y curl

FROM builder-base as builder

RUN curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.7/install.sh | bash

FROM builder

ARG NODE_VERSION=20

RUN nvm --version && \
    nvm install ${NODE_VERSION}
