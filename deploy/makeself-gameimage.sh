#!/usr/bin/env bash

######################################################################
# @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
# @file        : deploy
######################################################################

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

# Launcher does not need to be static since it runs inside the arch container
docker build . -t gameimage-launcher:arch -f deploy/Dockerfile.arch.launcher
docker run --rm -v "$(pwd)":/workdir gameimage-launcher:arch cp /dist/launcher /workdir
cp -fv ./launcher "$BIN_DIR"/gameimage-launcher
rm -f ./launcher

# Compile wizard, patch, and package with makeself
docker build . -t gameimage-wizard:alpine -f deploy/Dockerfile.alpine.wizard
docker run --rm -v "$(pwd)":/workdir gameimage-wizard:alpine cp -r /dist/makeself-wizard /workdir
cp -fr ./makeself-wizard/. "$BIN_DIR"

# Create backend
docker build . -t gameimage-backend:alpine -f deploy/Dockerfile.alpine.backend
docker run --rm -v "$(pwd)":/workdir gameimage-backend:alpine cp /dist/main /dist/boot /workdir
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

#
# Build
#

cd "$BUILD_DIR"

# Create runner script
{ sed -E 's/^\s+://' | tee "$BUILD_DIR"/app/start.sh; } <<-"END"
  :#!/bin/sh
  :
  :DIR_SCRIPT="${USER_PWD:+"$(pwd)"}"
  :DIR_SCRIPT="${DIR_SCRIPT:-"$(dirname -- "$(readlink -f "$0")")"}"
  :
  :DIR_CALL="${USER_PWD:-"$(pwd)"}"
  :DIR_BIN="$DIR_SCRIPT/bin"
  :
  :# Copy static bash
  :mkdir -p /tmp/gameimage/bin
  :cp "$DIR_SCRIPT/bin/bash" /tmp/gameimage/bin
  :
  :export PATH="$DIR_SCRIPT:$PATH"
  :export PATH="$DIR_BIN:$PATH"
  :export PATH="/tmp/gameimage/bin:$PATH"
  :
  :# Copy fonts
  :cp -r "$DIR_SCRIPT/usr" /tmp/gameimage
  :cp -r "$DIR_SCRIPT/etc" /tmp/gameimage
  :
  :export GIMG_BACKEND="$DIR_BIN/gameimage-cli"
  :
  :# Start application
  :cd "$DIR_CALL" && "$DIR_BIN"/wizard.sh "$@"
END
chmod +x "$BUILD_DIR"/app/start.sh

# Include fonts
# # Copy from container
mkdir -p "$BUILD_DIR"/app/usr/share
docker run -it --rm -v"$BUILD_DIR":"$BUILD_DIR" gameimage-backend:alpine cp -Lr /usr/share/fonts "$BUILD_DIR"/app/usr/share
docker run -it --rm -v"$BUILD_DIR":"$BUILD_DIR" gameimage-backend:alpine chown -R "$(id -u)":"$(id -u)" "$BUILD_DIR"/app/usr/share
mkdir -p "$BUILD_DIR"/app/etc
docker run -it --rm -v"$BUILD_DIR":"$BUILD_DIR" gameimage-backend:alpine cp -Lr /etc/fonts "$BUILD_DIR"/app/etc
docker run -it --rm -v"$BUILD_DIR":"$BUILD_DIR" gameimage-backend:alpine chown -R "$(id -u)":"$(id -u)" "$BUILD_DIR"/app/etc
# # Patch custom search path
sed -i 's|/usr/share/fonts|/tmp/gameimage/usr/share/fonts|' "$BUILD_DIR"/app/etc/fonts/fonts.conf
sed -i 's|/usr/local/share/fonts|/tmp/gameimage/usr/share/fonts|' "$BUILD_DIR"/app/etc/fonts/fonts.conf
sed -i 's|~/.fonts|/tmp/gameimage/usr/share/fonts|' "$BUILD_DIR"/app/etc/fonts/fonts.conf

# Package
mkdir -p tools
wget -O ./tools/makeself "https://github.com/ruanformigoni/makeself/releases/download/v2.5.1/makeself-2.5.1.run"
chmod +x ./tools/makeself
export MAKESELF_CLI_LOOP=1
./tools/makeself --target ./tools
chmod +x ./tools/*.sh
./tools/makeself.sh --xz "$BUILD_DIR/app" "gameimage.run" "Gameimage" './start.sh'
