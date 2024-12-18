FROM ubuntu:latest AS node-enviroment

RUN mkdir /nvm
ENV HOST 0.0.0.0

USER root

# Replace shell with bash so we can source files
RUN rm /bin/sh && ln -s /bin/bash /bin/sh

# Set debconf to run non-interactively
RUN echo 'debconf debconf/frontend select Noninteractive' | debconf-set-selections

# Install base dependencies
RUN apt-get update && apt-get install -y -q --no-install-recommends \
        apt-transport-https \
        build-essential \
        ca-certificates \
        curl \
        git \
        libssl-dev \
        wget \
        libpq-dev \
        clang \
        llvm pkg-config \
        nettle-dev \
        libc6-dev \
        libssl-dev \
    && apt-get upgrade -y \
    && rm -rf /var/lib/apt/lists/*

ENV NVM_DIR /tmp/.nvm
ENV NODE_VERSION 20
ENV NVM_ENV $NVM_DIR/nvm.sh
ENV NPM_ENV $NVM_ENV && nvm use $NODE_VERSION

RUN mkdir -p $NVM_DIR

# Install nvm with node, npm and yarn
RUN cd /tmp/ && wget https://raw.githubusercontent.com/creationix/nvm/v0.40.1/install.sh
RUN chmod +x /tmp/install.sh
RUN /tmp/install.sh

ENV NODE_PATH $NVM_DIR/v$NODE_VERSION/lib/node_modules
ENV PATH      $NVM_DIR/v$NODE_VERSION/bin:$PATH

FROM node-enviroment AS hookik-front

#install rust
RUN cd /tmp/ && curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

RUN cd /tmp/ && wget https://rustwasm.github.io/wasm-pack/installer/init.sh 

RUN chmod +x /tmp/init.sh

RUN . "$HOME/.cargo/env" && /tmp/init.sh 

RUN chmod +x $NVM_DIR/nvm.sh

ENV DATA_DIR="/data"
ENV RUST_LOG="hoodik=debug,auth=debug,error=debug,entity=debug,storage=debug,context=debug,util=debug,cryptfns=debug,actix_web=debug"

RUN cd /tmp/ \
    && wget https://raw.githubusercontent.com/creationix/nvm/v0.40.1/install.sh \
    && chmod +x install.sh \
    && ./install.sh

WORKDIR /hoodik

CMD . $HOME/.cargo/env \ 
    && . $NVM_DIR/nvm.sh \
    && cd /hoodik \
    && nvm install $NODE_VERSION \
    && nvm use $NODE_VERSION \
    && nvm alias default $NODE_VERSION \
    && npm install -g yarn \
    && yarn workspace @hoodik/cryptfns wasm-pack \
    && yarn workspace @hoodik/web dev --host

