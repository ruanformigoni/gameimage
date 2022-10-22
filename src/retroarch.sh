#!/usr/bin/env bash

######################################################################
# @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
# @file        : retroarch
# @created     : Monday Sep 19, 2022 20:24:27 -03
######################################################################

#shellcheck disable=2155
#shellcheck source=/dev/null

set -e

shopt -s globstar

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

source "$SCRIPT_DIR/common.sh"

function retroarch_download()
{
  # Get retroarch
  if [ ! -f "AppDir/usr/bin/retroarch" ]; then
    if [ ! -f "RetroArch-x86_64.AppImage" ]; then
      # Get AppImage of retroarch
      wget -q --show-progress --progress=bar:noscroll \
        "https://buildbot.libretro.com/nightly/linux/x86_64/RetroArch.7z"

      # Extract and move
      7z x "RetroArch.7z"
      mv RetroArch*/RetroArch*.AppImage retroarch.AppImage

      # Make executable
      chmod +x ./retroarch.AppImage

      # Extract
      ./retroarch.AppImage --appimage-extract

      # Erase downloaded appimage
      rm retroarch.AppImage

      # Erase problematic file
      rm squashfs-root/usr/lib/libwayland-client.so.0

      # Create new fixed appimage
      ARCH=x86_64 ./appimagetool squashfs-root
    fi

    cp RetroArch*.AppImage AppDir/usr/bin/retroarch
  fi
}

function runner_create()
{
  local bios="$(basename "$1")"
  local core="$(basename "$2")"
  local rom="$(basename "$3")"

  [ "$bios" == "null" ] && local bios=""

  # Create runner script
  { sed -E 's/^\s+://' | tee AppDir/AppRun; } <<-END
    :#!/usr/bin/env bash
    :
    :set -e
    :
    :# Set cfg dir
    :if [[ "\$(basename "\${APPIMAGE}")" =~ \.\.AppImage ]]; then
    :  # Set global
    :  export XDG_CONFIG_HOME="\$HOME/.config"
    :else
    :  # Set local
    :  export XDG_CONFIG_HOME="\$(dirname "\$APPIMAGE")/.\$(basename "\$APPIMAGE").config"
    :fi
    :
    :echo "XDG_CONFIG_HOME: \${XDG_CONFIG_HOME}"
    :
    :path_bios=\$XDG_CONFIG_HOME/retroarch/system/
    :
    :if [ "$bios" ] && [ ! -f "\${path_bios}/$bios" ]; then
    :  echo "bios: ${bios}"
    :  mkdir -p "\$path_bios"
    :  cp "\$APPDIR/app/$bios" "\$path_bios"
    :fi
    :
    :"\$APPDIR/usr/bin/retroarch" -L "\$APPDIR/app/${core}" "\$APPDIR/app/${rom}"
	END

  # Allow execute
  chmod +x AppDir/AppRun
}

function main()
{
  # Validate params
  readarray -t ret <<< "$(params_validate "retroarch" "$@")"

  [ "${ret[*]}" ] || exit 1

  local name="${ret[0]}"
  local dir="${ret[1]}"
  local bios="${ret[2]}"
  local core="${ret[3]}"
  local cover="${ret[4]}"
  local rom="${ret[5]}"

  # Create dirs
  cd "$(dir_build_create)"

  dir_appdir_create

  # Download tools
  appimagetool_download

  retroarch_download

  # Populate appdir
  files_copy "$name" "$dir" "$bios" "$core" "$cover"

  runner_create "$bios" "$core" "$rom"

  desktop_entry_create "$name"

  # Build appimage
  appdir_build
}

main "$@"
