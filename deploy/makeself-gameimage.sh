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
mkdir -p "$BUILD_DIR"

# Binaries location on package
BIN_DIR="$BUILD_DIR/app/bin"
mkdir -p "$BIN_DIR"

#
# Compile GUI
#

# Compile wizard, patch, and package with makeself
docker build . -t wizard:alpine -f deploy/Dockerfile.alpine.wizard
docker run --rm -v "$(pwd)":/workdir wizard:alpine cp -r /dist/makeself-wizard /workdir
cp -r ./makeself-wizard/. "$BIN_DIR"

# Launcher does not need to be static since it runs inside the arch container
docker build . -t launcher:alpine -f deploy/Dockerfile.arch.launcher
docker run --rm -v "$(pwd)":/workdir launcher:alpine cp /dist/launcher /workdir
cp ./launcher "$BIN_DIR"
rm -f ./launcher

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

# Export to use them to build
export PATH="$BIN_DIR:$PATH"

# Copy files
cp -r ./src/* "$BUILD_DIR"/app/bin
cp    ./doc/gameimage.png "$BUILD_DIR"/app

for i in "$BUILD_DIR"/app/bin/*; do
  echo "$i"
  chmod +x "$i"
done

# Resize image
wget -O magick https://github.com/ruanformigoni/imagemagick-static-musl/releases/download/cc3f21c/magick-x86_64
chmod +x magick
./magick "$BUILD_DIR"/app/gameimage.png -resize 200x200 "$BUILD_DIR"/app/gameimage.png

#
# Build
#

cd "$BUILD_DIR"

# Create runner script
{ sed -E 's/^\s+://' | tee "$BUILD_DIR"/app/start.sh; } <<-"END"
  :#!/bin/sh
  :
  :# In makeself the extracted directory is the initial reference
  :PATH_SCRIPT="$(pwd)"
  :
  :export PATH="$PATH_SCRIPT:$PATH"
  :export PATH="$PATH_SCRIPT/bin:$PATH"
  :export PATH="/tmp/gameimage/bin:$PATH"
  :
  :mkdir -p /tmp/gameimage/bin
  :
  :# Copy static bash
  :cp "$PATH_SCRIPT/bin/bash" /tmp/gameimage/bin
  :
  :# Copy fonts
  :cp -r "$PATH_SCRIPT/usr" /tmp/gameimage
  :cp -r "$PATH_SCRIPT/etc" /tmp/gameimage
  :
  :# Copy icon
  :cp "$PATH_SCRIPT/gameimage.png" /tmp/gameimage/gameimage.png
  :
  :main.sh "$@"
END
chmod +x "$BUILD_DIR"/app/start.sh

# Include fonts
mkdir -p "$BUILD_DIR"/app/usr/share
cp -Lr /usr/share/fonts "$BUILD_DIR"/app/usr/share
mkdir -p "$BUILD_DIR"/app/etc
cp -Lr /etc/fonts "$BUILD_DIR"/app/etc
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
