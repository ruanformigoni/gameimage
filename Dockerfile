FROM ubuntu:latest

# Env
ARG DEBIAN_FRONTEND=noninteractive
ENV TZ=America/New_York
 
# Install packages
RUN apt update
RUN apt install -y git wget curl patchelf python3-pip file build-essential cmake \
  libfuse3-dev libxinerama-dev libxcursor-dev libxfixes-dev libxft-dev libpango-1.0-0 \
  libpango1.0-dev libpangoxft-1.0-0 libpangocairo-1.0-0 libgtk-3-dev \
  libappindicator3-1 libgail-dev libgail-3-dev libxapp-dev gvfs-libs

# Fetch source
# RUN mkdir -p gameimage
# COPY . gameimage
# WORKDIR gameimage
# RUN sed -i "s/TRUNK/$CI_COMMIT_TAG/" src/main.sh
RUN git clone https://gitlab.com/formigoni/gameimage.git
WORKDIR gameimage
RUN sed -i "s/TRUNK/$CI_COMMIT_TAG/" src/main.sh

# Build application
RUN pip3 install staticx
RUN apt install -y curl
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
RUN PATH="$PATH:$HOME/.cargo/bin" ./deploy/deploy.sh
