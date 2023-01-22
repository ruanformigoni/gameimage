#!/usr/bin/env bash

######################################################################
# @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
# @file        : deploy
######################################################################

set -e

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

# Compile gui
cd gui/wizard && cargo build --release

cd "$(dirname "$SCRIPT_DIR")"

cd gui/menu-button && cargo build --release

cd "$(dirname "$SCRIPT_DIR")"

# Create build dir
mkdir -p build && cd build

# Create appdir
mkdir -p AppDir/usr/bin

# Fetch yq
wget -q --show-progress --progress=dot:mega https://github.com/mikefarah/yq/releases/download/v4.30.7/yq_linux_amd64.tar.gz -O - | tar xz
rm yq.1
mv yq_linux_amd64 yq
mv yq AppDir/usr/bin

# Copy files
cp -r ../src/* AppDir/usr/bin
cp -r ../doc/gameimage.png AppDir/
cp -r ../gui/wizard/target/release/gameimage-install-gui AppDir/usr/bin/gui
cp -r ../gui/menu-button/target/release/menu-button AppDir/usr/bin/menu-button

for i in AppDir/usr/bin/*; do
  echo "$i"
  chmod +x "$i"
done

# Create runner script
{ sed -E 's/^\s+://' | tee AppDir/AppRun; } <<-END
  :#!/usr/bin/env bash
  :
  :\$APPDIR/usr/bin/main.sh "\$@"
END
chmod +x AppDir/AppRun

# Create desktop entry
{ sed -E 's/^\s+://' | tee AppDir/gameimage.desktop; } <<-END
  :[Desktop Entry]
  :Name=GameImage
  :Exec=/usr/bin/bash
  :Icon=gameimage
  :Type=Application
  :Categories=Utility;
END

# Download appimagetool
[ ! -f "appimagetool" ] && wget -q --show-progress --progress=bar:noscroll -O appimagetool https://github.com/AppImage/AppImageKit/releases/download/continuous/appimagetool-x86_64.AppImage

chmod +x appimagetool

# Extract
./appimagetool --appimage-extract

# Package
ARCH=x86_64 ./squashfs-root/AppRun AppDir
