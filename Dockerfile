FROM ubuntu:20.04

ENV SHELL /bin/bash
ENV DEBIAN_FRONTEND noninteractive

## build essentials for c++
RUN apt-get update && apt-get install -y \
    git wget unzip sudo default-jre && \
    # install simulater
    wget "https://drive.google.com/uc?export=download&id=1xZzcMIUETv3u3sdpM_oTJSTetpVee3KZ" -O nand2tetris.zip && \
    unzip nand2tetris.zip && \
    chmod -R +x /nand2tetris && \
    # clean-up
    apt-get autoremove -y && \
    apt-get clean &&\
    rm -rf /var/lib/apt/lists/* && \
    rm -rf /usr/local/src/*

CMD ["/bin/bash"]