#!/usr/bin/env bash

######################################################################
# @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
# @file        : deploy
######################################################################

set -e

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

# Create build dir and appdir
mkdir -p build/AppDir/usr/bin

# Compile gui
cd gui/wizard && cargo build --release
cd "$(dirname "$SCRIPT_DIR")"
staticx \
  -l "/usr/lib/x86_64-linux-gnu/libpthread.so.0" \
  -l "/usr/lib/x86_64-linux-gnu/libappindicator3.so.1" \
  -l "/lib/x86_64-linux-gnu/libdbusmenu-glib.so.4" \
  -l "/lib/x86_64-linux-gnu/libdbusmenu-gtk3.so.4" \
  -l "/usr/lib/x86_64-linux-gnu/gio/modules/libdconfsettings.so" \
  -l "/lib/x86_64-linux-gnu/libdl.so.2" \
  -l "/usr/lib/x86_64-linux-gnu/gio/modules/libgioremote-volume-monitor.so" \
  -l "/lib/x86_64-linux-gnu/libgnomekbd.so.8" \
  -l "/lib/x86_64-linux-gnu/libgnomekbdui.so.8" \
  -l "/usr/lib/x86_64-linux-gnu/gvfs/libgvfscommon.so" \
  -l "/usr/lib/x86_64-linux-gnu/gio/modules/libgvfsdbus.so" \
  -l "/lib/x86_64-linux-gnu/libstdc++.so.6" \
  -l "/lib/x86_64-linux-gnu/libxapp.so.1" \
  -l "/lib/x86_64-linux-gnu/libxkbfile.so.1" \
  -l "/lib/x86_64-linux-gnu/libxklavier.so.16" \
  -l "/lib/x86_64-linux-gnu/libxml2.so.2" \
  ./gui/wizard/target/release/gameimage-install-gui build/AppDir/usr/bin/gui-installer

# Compile launcher
cd gui/launcher && cargo build --release
cd "$(dirname "$SCRIPT_DIR")"
staticx \
  -l "/usr/lib/x86_64-linux-gnu/libpthread.so.0" \
  -l "/usr/lib/x86_64-linux-gnu/libappindicator3.so.1" \
  -l "/lib/x86_64-linux-gnu/libdbusmenu-glib.so.4" \
  -l "/lib/x86_64-linux-gnu/libdbusmenu-gtk3.so.4" \
  -l "/usr/lib/x86_64-linux-gnu/gio/modules/libdconfsettings.so" \
  -l "/lib/x86_64-linux-gnu/libdl.so.2" \
  -l "/usr/lib/x86_64-linux-gnu/gio/modules/libgioremote-volume-monitor.so" \
  -l "/lib/x86_64-linux-gnu/libgnomekbd.so.8" \
  -l "/lib/x86_64-linux-gnu/libgnomekbdui.so.8" \
  -l "/usr/lib/x86_64-linux-gnu/gvfs/libgvfscommon.so" \
  -l "/usr/lib/x86_64-linux-gnu/gio/modules/libgvfsdbus.so" \
  -l "/lib/x86_64-linux-gnu/libstdc++.so.6" \
  -l "/lib/x86_64-linux-gnu/libxapp.so.1" \
  -l "/lib/x86_64-linux-gnu/libxkbfile.so.1" \
  -l "/lib/x86_64-linux-gnu/libxklavier.so.16" \
  -l "/lib/x86_64-linux-gnu/libxml2.so.2" \
  ./gui/launcher/target/release/gameimage-launcher build/AppDir/usr/bin/gui-launcher

# Include shared version
cp ./gui/launcher/target/release/gameimage-launcher build/AppDir/usr/bin/gui-launcher-shared

# Fetch unionfs
wget -q --show-progress --progress=dot:mega https://github.com/ruanformigoni/unionfs-fuse/releases/download/ebac73a/unionfs
mv -f unionfs build/AppDir/usr/bin

# Fetch overlayfs
wget -q --show-progress --progress=dot:mega https://github.com/ruanformigoni/fuse-overlayfs/releases/download/af507a2/fuse-overlayfs-x86_64
mv -f fuse-overlayfs-x86_64 build/AppDir/usr/bin/overlayfs

# Fetch yq
wget -q --show-progress --progress=dot:mega https://github.com/mikefarah/yq/releases/download/v4.30.7/yq_linux_amd64.tar.gz -O - | tar xz
rm yq.1
mv yq_linux_amd64 yq
mv yq build/AppDir/usr/bin

# Fetch jq
wget -q --show-progress --progress=dot:binary https://github.com/jqlang/jq/releases/download/jq-1.7/jq-linux-amd64 -O build/AppDir/usr/bin/jq

# Fetch 7zz
wget -q --show-progress --progress=dot:mega https://github.com/ruanformigoni/7zip_static/releases/download/ed1f3df/7zz
mv -f 7zz build/AppDir/usr/bin/7zz

# Fetch busybox
wget -q --show-progress --progress=dot:mega "https://www.busybox.net/downloads/binaries/1.35.0-x86_64-linux-musl/busybox"
mv -f busybox build/AppDir/usr/bin/busybox

# Fetch fd
basename_fd="fd-v8.7.1-x86_64-unknown-linux-musl"
wget -q --show-progress --progress=dot:mega "https://github.com/sharkdp/fd/releases/download/v8.7.1/${basename_fd}.tar.gz"
tar -xf "${basename_fd}.tar.gz"
mv -f "${basename_fd}/fd" build/AppDir/usr/bin/fd
rm -rf ./fd-*

# Fetch aria2c
wget -q --show-progress --progress=dot:mega "https://github.com/ruanformigoni/aria2-static-musl/releases/download/2d7f402/aria2c" -o build/AppDir/usr/bin/aria2c

# Copy files
cp -r ./src/* build/AppDir/usr/bin
cp    ./doc/gameimage.png build/AppDir/

for i in build/AppDir/usr/bin/*; do
  echo "$i"
  chmod +x "$i"
done

# Create runner script
{ sed -E 's/^\s+://' | tee build/AppDir/AppRun; } <<-END
  :#!/usr/bin/env bash
  :
  :\$APPDIR/usr/bin/main.sh "\$@"
END
chmod +x build/AppDir/AppRun

# Create desktop entry
{ sed -E 's/^\s+://' | tee build/AppDir/gameimage.desktop; } <<-END
  :[Desktop Entry]
  :Name=GameImage
  :Exec=/usr/bin/bash
  :Icon=gameimage
  :Type=Application
  :Categories=Utility;
END

# Enter build dir
cd build

# Download appimagetool
[ ! -f "appimagetool" ] && wget -q --show-progress --progress=bar:noscroll -O appimagetool https://github.com/AppImage/AppImageKit/releases/download/continuous/appimagetool-x86_64.AppImage

chmod +x appimagetool

# Extract
./appimagetool --appimage-extract

# Package
ARCH=x86_64 ./squashfs-root/AppRun AppDir
