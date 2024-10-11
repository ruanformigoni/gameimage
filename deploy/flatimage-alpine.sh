#!/usr/bin/env bash

######################################################################
# @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
# @file        : flatimage-alpine
# @created     : Sunday Sep 08, 2024 12:12:19 -03
#
# @description : 
######################################################################

#shellcheck disable=2016

set -e

#
# Setup environment
#

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

# Reference source dir
SRC_DIR="$(dirname "$SCRIPT_DIR")"
cd "$SRC_DIR"

# Create build dir and appdir
BUILD_DIR="$SRC_DIR/build"
rm -rf "$BUILD_DIR" && mkdir "$BUILD_DIR"

# Binaries location on package
BIN_DIR="$BUILD_DIR/app/bin"
mkdir -p "$BIN_DIR"

#
# Compile GUI
#

# Compile wizard
docker build . -t gameimage-wizard-build -f deploy/Dockerfile.wizard.build
docker run --rm -v "$(pwd)":/workdir gameimage-wizard-build cp /dist/wizard /workdir
cp -vf ./wizard "$BIN_DIR"/gameimage-wizard
rm -vf ./wizard

# Launcher does not need to be static since it runs inside the arch container
docker build . -t gameimage-launcher-build -f deploy/Dockerfile.launcher.build
docker run --rm -v "$(pwd)":/workdir gameimage-launcher-build cp /dist/launcher /workdir
cp -vf ./launcher "$BIN_DIR"/gameimage-launcher
rm -vf ./launcher

# Create backend
docker build . -t gameimage-backend-build -f deploy/Dockerfile.backend.build
docker run --rm -v "$(pwd)":/workdir gameimage-backend-build cp /dist/main /dist/boot /workdir
cp -fv ./main "$BIN_DIR"/gameimage-cli
cp -fv ./boot "$BIN_DIR"/gameimage-boot
rm -f ./main ./boot

#
# Fetch tools
#

function _fetch()
{
  local link="$1"
  local out="$2"
  local file="$3"
  if [[ "$link" =~ .tar.(xz|gz) ]]; then
    wget -q --show-progress --progress=dot:mega "$link" -O - | tar xz "$file" -O > "$out"
  else
    wget -q --show-progress --progress=dot:mega "$link" -O "$out"
  fi
  chmod +x "$out"
}

# Fetch unionfs
_fetch "https://github.com/ruanformigoni/unionfs-fuse/releases/download/ebac73a/unionfs" "$BIN_DIR"/unionfs

# Fetch overlayfs
_fetch "https://github.com/ruanformigoni/fuse-overlayfs/releases/download/af507a2/fuse-overlayfs-x86_64" "$BIN_DIR"/overlayfs

# Fetch yq
_fetch "https://github.com/mikefarah/yq/releases/download/v4.30.7/yq_linux_amd64.tar.gz" "$BIN_DIR/yq" "./yq_linux_amd64"

# Fetch jq
_fetch "https://github.com/jqlang/jq/releases/download/jq-1.7/jq-linux-amd64" "$BIN_DIR"/jq

# Fetch 7zz
_fetch "https://github.com/ruanformigoni/7zip_static/releases/download/ed1f3df/7zz" "$BIN_DIR"/7zz

# Fetch busybox
_fetch "https://www.busybox.net/downloads/binaries/1.35.0-x86_64-linux-musl/busybox" "$BIN_DIR"/busybox

_fetch "https://github.com/ruanformigoni/imagemagick-static-musl/releases/download/c1c5775/magick-x86_64" "$BIN_DIR"/magick

# Fetch lsof
_fetch "https://github.com/ruanformigoni/lsof-static-musl/releases/download/12c2552/lsof-x86_64" "$BIN_DIR"/lsof

# Fetch fd
_fetch "https://github.com/sharkdp/fd/releases/download/v8.7.1/fd-v8.7.1-x86_64-unknown-linux-musl.tar.gz" "$BIN_DIR"/fd "fd-v8.7.1-x86_64-unknown-linux-musl/fd"

# Fetch aria2
_fetch "https://github.com/ruanformigoni/aria2-static-musl/releases/download/2d7f402/aria2c" "$BIN_DIR"/aria2c

# Fetch bash
_fetch "https://github.com/ruanformigoni/bash-static/releases/download/8ba11cd/bash-x86_64" "$BIN_DIR"/bash

# Fetch coreutils
wget -q --show-progress --progress=dot:mega "https://github.com/ruanformigoni/coreutils-static/releases/download/d7f4cd2/coreutils-x86_64.tar.xz"
tar -xf "coreutils-x86_64.tar.xz" -C"$BIN_DIR" --strip-components=1
rm "coreutils-x86_64.tar.xz"

# Fetch sed
_fetch "https://github.com/ruanformigoni/gnu-static-musl/releases/download/b122ecc/sed" "$BIN_DIR"/sed

# Fetch grep
_fetch "https://github.com/ruanformigoni/gnu-static-musl/releases/download/b122ecc/grep" "$BIN_DIR"/grep

# Fetch tar
_fetch "https://github.com/ruanformigoni/gnu-static-musl/releases/download/b122ecc/tar" "$BIN_DIR"/tar

# Fetch xz
_fetch "https://github.com/ruanformigoni/xz-static-musl/releases/download/fec8a15/xz" "$BIN_DIR"/xz

# Fetch pv
_fetch "https://github.com/ruanformigoni/pv-static-musl/releases/download/3398ec0/pv-x86_64" "$BIN_DIR"/pv

# Export to use them to build
export PATH="$BIN_DIR:$PATH"

# Copy files
for i in "$BUILD_DIR"/app/bin/*; do
  echo "$i"
  chmod +x "$i"
done

# TODO Fetch from github
# Fetch flatimage
export IMAGE="$BUILD_DIR"/alpine.flatimage
cp ~/Public/alpine.flatimage "$IMAGE"

# Set permissions
"$IMAGE" fim-perms add home,media,network,xorg,wayland

# Install dependencies
"$IMAGE" fim-root apk add libxkbcommon libxinerama libxcursor font-noto xz tar libssl3

# Set environment variables
"$IMAGE" fim-env set 'PATH=/opt/gameimage/bin:"$PATH"' 'GIMG_BACKEND="/opt/gameimage/bin/gameimage-cli"'

# Set boot command
"$IMAGE" fim-boot sh -c '/opt/gameimage/bin/gameimage-wizard'

# Copy binaries
"$IMAGE" fim-exec cp -r "$BUILD_DIR"/app /opt/gameimage

# Create novel image layer
"$IMAGE" fim-commit

mv "$IMAGE" "$BUILD_DIR"/gameimage

#  vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
