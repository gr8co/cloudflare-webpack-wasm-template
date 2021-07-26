# See here for image contents: https://github.com/microsoft/vscode-dev-containers/tree/v0.166.0/containers/rust/.devcontainer/base.Dockerfile

FROM mcr.microsoft.com/vscode/devcontainers/rust:0-1
ENV NODE_VER=v14.17.3
RUN curl -fsSL https://download.docker.com/linux/debian/gpg | gpg --dearmor -o /usr/share/keyrings/docker-archive-keyring.gpg &&\
    echo "deb [arch=amd64 signed-by=/usr/share/keyrings/docker-archive-keyring.gpg] https://download.docker.com/linux/debian $(lsb_release -cs) stable" | tee /etc/apt/sources.list.d/docker.list > /dev/null &&\
    apt-get update && apt-get install -qy docker-ce-cli &&\
    curl -L "https://github.com/docker/compose/releases/download/1.29.0/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose &&\
    chmod +x /usr/local/bin/docker-compose &&\
    groupadd docker &&\
    usermod -G docker vscode &&\
    mkdir -p /usr/local/lib/nodejs &&\
    wget https://nodejs.org/dist/${NODE_VER}/node-${NODE_VER}-linux-x64.tar.xz &&\
    tar -xJvf node-${NODE_VER}-linux-x64.tar.xz -C /usr/local/lib/nodejs &&\
    rm -f node-*.tar.xz
USER vscode
ENV PATH="/usr/local/lib/nodejs/node-${NODE_VER}-linux-x64/bin:${PATH}"
RUN rustup update stable && rustup default stable &&\
    rustup component add rust-src &&\
    cargo install wrangler &&\
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# [Optional] Uncomment this section to install additional packages.
# RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
#     && apt-get -y install --no-install-recommends <your-package-list-here>
