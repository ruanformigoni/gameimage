#!/usr/bin/env bash

######################################################################
# @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
# @file        : install
# @created     : Monday Sep 12, 2022 22:02:01 -03
######################################################################

set -e

# Install required packages
if command -v apt; then
  sudo apt install -y tumbler squashfs-tools thunar
elif command -v pacman; then
  sudo pacman -S --noconfirm tumbler squashfs-tools thunar
else
  echo "Unsupported package manager, please install the packages manually"
  exit 1
fi

# Copy thumbnail generator
sudo curl --output /usr/bin/thumbnailer-appimage https://gitlab.com/formigoni/agape/-/raw/master/thumbnailer/thumbnailer-appimage
sudo chmod +x /usr/bin/thumbnailer-appimage

# Copy tumbler entry
mkdir -p ~/.local/share/thumbnailers
curl --output ~/.local/share/thumbnailers/appimage.thumbnailer \
  https://gitlab.com/formigoni/agape/-/raw/master/thumbnailer/appimage.thumbnailer

# Remove 2GB limit on thumbnails
mkdir -p ~/.config/tumbler
cp /etc/xdg/tumbler/tumbler.rc ~/.config/tumbler/
awk -i inplace '/\[DesktopThumbnailer\]/,/MaxFileSize=(.*)/ { sub("MaxFileSize=.*", "MaxFileSize="); } 1' ~/.config/tumbler/tumbler.rc
