#!/usr/bin/env bash

######################################################################
# @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
# @file        : deploy
######################################################################

set -e

#
# Setup environment
#

# Enable community repo
echo https://dl-cdn.alpinelinux.org/alpine/edge/main/ > /etc/apk/repositories
echo https://dl-cdn.alpinelinux.org/alpine/edge/community/ >> /etc/apk/repositories
echo https://dl-cdn.alpinelinux.org/alpine/edge/testing/ >> /etc/apk/repositories

# Install packages
apk update
apk add git wget curl patchelf py3-pip pipx file build-base cmake \
  fuse3-dev libxinerama-dev libxcursor-dev libxfixes-dev libxft-dev pango \
  pango-dev libayatana-appindicator libayatana-appindicator-dev gtk+3.0-dev \
  bash fuse fuse3

# Build rust
curl https://sh.rustup.rs -sSf | sh -s -- -y
export PATH="$HOME/.cargo/bin:$PATH"

# Symlink compilers
ln -sfT /usr/bin/gcc /usr/bin/musl-gcc
ln -sfT /usr/bin/g++ /usr/bin/musl-g++

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

# Reference source dir
SRC_DIR="$(dirname "$SCRIPT_DIR")"
cd "$SRC_DIR"

# Create build dir and appdir
BUILD_DIR="$SRC_DIR/build"
mkdir -p "$BUILD_DIR"

# Create AppDir
mkdir -p "$BUILD_DIR/AppDir/usr/bin"

#
# Compile GUI
#

# Compile as shared libraries
export RUSTFLAGS='-C target-feature=-crt-static'

# Compile gui & launcher
( cd gui/wizard && cargo build --release )
( cd gui/launcher && cargo build --release )

# Include shared version
cp ./gui/launcher/target/release/gameimage-launcher  build/AppDir/usr/bin/gui-launcher
cp ./gui/wizard/target/release/gameimage-install-gui build/AppDir/usr/bin/gui-installer

#
# Fetch tools
#

# Fetch unionfs
wget -q --show-progress --progress=dot:mega https://github.com/ruanformigoni/unionfs-fuse/releases/download/ebac73a/unionfs \
  -O "$BUILD_DIR"/AppDir/usr/bin/unionfs

# Fetch overlayfs
wget -q --show-progress --progress=dot:mega \
  https://github.com/ruanformigoni/fuse-overlayfs/releases/download/af507a2/fuse-overlayfs-x86_64 \
  -O "$BUILD_DIR"/AppDir/usr/bin/overlayfs

# Fetch yq
wget -q --show-progress --progress=dot:mega https://github.com/mikefarah/yq/releases/download/v4.30.7/yq_linux_amd64.tar.gz -O - | tar xz
rm yq.1
mv ./yq_linux_amd64 "$BUILD_DIR"/AppDir/usr/bin/yq

# Fetch jq
wget -q --show-progress --progress=dot:binary https://github.com/jqlang/jq/releases/download/jq-1.7/jq-linux-amd64 \
  -O "$BUILD_DIR"/AppDir/usr/bin/jq

# Fetch 7zz
wget -q --show-progress --progress=dot:mega https://github.com/ruanformigoni/7zip_static/releases/download/ed1f3df/7zz \
  -O "$BUILD_DIR"/AppDir/usr/bin/7zz

# Fetch busybox
wget -q --show-progress --progress=dot:mega "https://www.busybox.net/downloads/binaries/1.35.0-x86_64-linux-musl/busybox" \
  -O "$BUILD_DIR"/AppDir/usr/bin/busybox

# Fetch aria2
wget -q --show-progress --progress=dot:mega "https://github.com/ruanformigoni/aria2-static-musl/releases/download/2d7f402/aria2c" \
  -O "$BUILD_DIR"/AppDir/usr/bin/aria2c

# Copy files
cp -r ./src/* "$BUILD_DIR"/AppDir/usr/bin
cp    ./doc/gameimage.png "$BUILD_DIR"/AppDir/

for i in "$BUILD_DIR"/AppDir/usr/bin/*; do
  echo "$i"
  chmod +x "$i"
done

#
# Build flatimage
#

# Fetch flatimage
if [[ ! -f ./alpine.tar.xz ]]; then
  wget "https://gitlab.com/api/v4/projects/43000137/packages/generic/fim/continuous/alpine.tar.xz"
fi
tar xf alpine.tar.xz

# Perms
./alpine.fim fim-perms-set wayland,x11,session_bus

# Resize
./alpine.fim fim-resize 500M

# Install runtime deps
./alpine.fim fim-root apk add libxkbcommon-dev libxkbcommon libxinerama libxcursor \
  gtk+3.0 libayatana-appindicator

# Create runner script
{ sed -E 's/^\s+://' | tee "$BUILD_DIR"/AppDir/AppRun; } <<-"END"
  :#!/usr/bin/env bash
  :
  :PATH="$FIM_DIR_GLOBAL_BIN:$FIM_DIR_STATIC:$FIM_DIR_MOUNT/app/usr/bin:$PATH"
  :
  :"$FIM_DIR_MOUNT/app/usr/bin/main.sh" "$@"
END
chmod +x "$BUILD_DIR"/AppDir/AppRun

# Include application
./alpine.fim fim-include-path "$BUILD_DIR/AppDir" "/app"

# Set default command
./alpine.fim fim-cmd '$FIM_DIR_MOUNT/app/AppRun'
