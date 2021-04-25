# This is a small builder image to enable the build of rust and nodejs projects with wrangler
# Used by Github Action
FROM rust:1-slim
ENV NODE_VER=v14.16.1
ENV PATH="/usr/local/lib/nodejs/node-${NODE_VER}-linux-x64/bin:${PATH}"
RUN apt update && apt install -y curl wget xz-utils perl make &&\
    mkdir -p /usr/local/lib/nodejs &&\
    wget https://nodejs.org/dist/${NODE_VER}/node-${NODE_VER}-linux-x64.tar.xz &&\
    tar -xJvf node-${NODE_VER}-linux-x64.tar.xz -C /usr/local/lib/nodejs &&\
    rm -f node-*.tar.xz &&\
    cargo install wrangler &&\
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh &&\
    apt remove -y wget xz-utils
