FROM alpine:latest

# Enable community repo
RUN echo https://dl-cdn.alpinelinux.org/alpine/edge/main/ > /etc/apk/repositories
RUN echo https://dl-cdn.alpinelinux.org/alpine/edge/community/ >> /etc/apk/repositories
RUN echo https://dl-cdn.alpinelinux.org/alpine/edge/testing/ >> /etc/apk/repositories

# Install deps
RUN apk update && apk upgrade
RUN apk add --no-cache build-base git libbsd-dev py3-pip pipx git patchelf cmake gcc \
  bash e2fsprogs xz curl zstd gawk debootstrap m4 gcompat nasm

# Install conan
RUN pipx install conan

# Update PATH
ENV PATH="/root/.local/bin:$PATH"

# Setup
RUN conan profile detect --force

# Copy files
RUN mkdir /gameimage
COPY . /gameimage/

# Set workdir
WORKDIR /gameimage

# Compile
RUN conan install . --build=missing -g CMakeDeps -g CMakeToolchain --output-folder=build || true
# nasm bug when on musl system "undefined symbol"
RUN cp "$(command -v nasm)" /root/.conan2/p/nasmc*/p/bin/nasm
RUN conan install . --build=missing -g CMakeDeps -g CMakeToolchain --output-folder=build || true
RUN cmake --preset conan-release -DCMAKE_BUILD_TYPE=Release
RUN cmake --build --preset conan-release

# Build image
# RUN cp dist/main bin/elf
# RUN ./src/scripts/_build.sh debootstrap focal
# RUN ./src/scripts/_build.sh archbootstrap
# RUN ./src/scripts/_build.sh alpinebootstrap
