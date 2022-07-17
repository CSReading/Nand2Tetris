FROM ubuntu:20.04

ENV SHELL /bin/bash
ENV DEBIAN_FRONTEND noninteractive

## build nand2tetris
RUN apt update && apt install -y \
    git wget unzip sudo curl build-essential default-jre && \
    # install simulater
    wget "https://drive.google.com/uc?export=download&id=1xZzcMIUETv3u3sdpM_oTJSTetpVee3KZ" -O nand2tetris.zip && \
    unzip nand2tetris.zip && \
    chmod -R +x /nand2tetris && \
    # install Rust
    curl https://sh.rustup.rs -sSf | sh -s -- -y && \
    chmod -R +rx /root && \ 
    # clean-up
    apt-get autoremove -y && \
    apt-get clean &&\
    rm -rf /var/lib/apt/lists/* && \
    rm -rf /usr/local/src/*
# setup Rust
ENV PATH="/root/.cargo/bin:$PATH"
# need "rustup update stable" in container

ARG USERNAME=vscode
ARG USER_UID=1000
ARG USER_GID=$USER_UID

# Create the user
RUN groupadd --gid $USER_GID $USERNAME \
    && useradd --uid $USER_UID --gid $USER_GID -m $USERNAME

USER $USERNAME

CMD ["/bin/bash"]