#!/tmp/gameimage/bin/bash

######################################################################
# @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
# @file        : retroarch
# @created     : Monday Sep 19, 2022 20:24:27 -03
######################################################################

#shellcheck disable=2155
#shellcheck source=/dev/null

set -e

shopt -s globstar

GIMG_SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

source "$GIMG_SCRIPT_DIR/common.sh"

function retroarch_download()
{
  local url

  # Get retroarch url
  if [[ "$GIMG_PKG_TYPE" = "flatimage" ]]; then
    url="$(_fetch_stdout "https://api.github.com/repos/flatimage/flatimage-retroarch/releases/latest" \
      | jq -r ".assets.[0].browser_download_url")"
  else
    url="https://buildbot.libretro.com/nightly/linux/x86_64/RetroArch.7z"
  fi

  if [ ! -f "AppDir/usr/bin/retroarch" ]; then
    msg "retroarch: $url"

    if [[ "$GIMG_PKG_TYPE" = "flatimage" ]]; then
      _fetch "AppDir/usr/bin/retroarch" "$url"
      # Set home directory to build dir
      ./AppDir/usr/bin/retroarch fim-config-set home "$DIR_SRC"
    else
      # AppImage requires additional patching to avoid crashes

      # Get AppImage of retroarch
      _fetch "RetroArch.7z" "$url"

      # Extract and move
      "$GIMG_SCRIPT_DIR"/7zz x "RetroArch.7z"
      mv RetroArch-Linux-x86_64/RetroArch*.AppImage retroarch.AppImage
      mv RetroArch-Linux-x86_64/RetroArch*.AppImage.home/.config config
      rm -rf RetroArch-Linux-x86_64

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

      # Remove extract dir
      rm -rf squashfs-root

      # Copy assets into the AppImage
      cp -r config AppDir/app/config 

      # Copy retroarch into the appimage
      cp RetroArch*.AppImage AppDir/usr/bin/retroarch
    fi
  fi
}

function runner_create()
{
  local name="$1"
  local bios="$(basename "$2")"
  local rom="$(basename "$3")"
  local core="$(basename "$4")"

  [ "$bios" == "null" ] && local bios=""

  # Define common variables for each package type
  # shellcheck disable=2016
  if [[ "$GIMG_PKG_TYPE" = "flatimage" ]]; then
    export RUNNER_PATH='/fim/shared:/retroarch/bin:$PATH'
    export RUNNER_XDG_CONFIG_HOME='${FIM_DIR_BINARY}/.${FIM_FILE_BINARY}.config/overlays/app/mount/xdg/config'
    export RUNNER_XDG_DATA_HOME='${FIM_DIR_BINARY}/.${FIM_FILE_BINARY}.config/overlays/app/mount/xdg/data'
    export RUNNER_MOUNTPOINT='$FIM_DIR_MOUNT'
    export RUNNER_ASSETS=/assets/.config/retroarch
    export RUNNER_BIN='/fim/scripts/retroarch.sh'
    export RUNNER_LAUNCHER_IMG='$FIM_DIR_MOUNT/fim/desktop/icon.png'
  else
    export RUNNER_PATH='$APPDIR/usr/bin:$PATH'
    export RUNNER_XDG_CONFIG_HOME='$(dirname "$APPIMAGE")/.$(basename "$APPIMAGE").config/xdg/config'
    export RUNNER_XDG_DATA_HOME='$(dirname "$APPIMAGE")/.$(basename "$APPIMAGE").config/xdg/data'
    export RUNNER_MOUNTPOINT='$APPDIR'
    export RUNNER_ASSETS='$APPDIR/app/config/retroarch'
    export RUNNER_BIN='$APPDIR/usr/bin/retroarch'
  fi
  
  # Create runner script
  { sed -E 's/^\s+://' | tee AppDir/AppRun | sed -e 's/^/-- /'; } <<-END
    :#!/usr/bin/env bash
    :
    :set -e
    :
    :SCRIPT_NAME="\$(basename "\$0")"
    :
    :exec 1> >(while IFS= read -r line; do echo "[\$SCRIPT_NAME] \$line"; done)
    :exec 2> >(while IFS= read -r line; do echo "[\$SCRIPT_NAME] \$line" >&2; done)
    :
    :# PATH
    :export PATH="$RUNNER_PATH"
    :
    :# Platform
    :export GIMG_PLATFORM=$GIMG_PLATFORM
    :echo "GIMG_PLATFORM: \${GIMG_PLATFORM}"
    :
    :# Package Type
    :export GIMG_PKG_TYPE=$GIMG_PKG_TYPE
    :echo "GIMG_PKG_TYPE: \${GIMG_PKG_TYPE}"
    :
    :# Set cfg dir
    :export XDG_CONFIG_HOME="$RUNNER_XDG_CONFIG_HOME"
    :echo "XDG_CONFIG_HOME: \${XDG_CONFIG_HOME}"
    :
    :# Set data dir
    :export XDG_DATA_HOME="$RUNNER_XDG_DATA_HOME"
    :echo "XDG_DATA_HOME: \${XDG_DATA_HOME}"
    :
    :# Runner binary path
    :echo "RUNNER_BIN: $RUNNER_BIN"
    :
    :# Check if retroarch assets are missing
    :dir_retroarch_assets="\$XDG_CONFIG_HOME/retroarch"
    :if [ ! -d "\$dir_retroarch_assets" ]; then
    :  mkdir -p "\$dir_retroarch_assets"
    :  cp -r "$RUNNER_ASSETS"/. "\$dir_retroarch_assets"
    :fi
    :
    :path_bios=\$XDG_CONFIG_HOME/retroarch/system/
    :
    :if [ "$bios" ] && [ ! -f "\${path_bios}/$bios" ]; then
    :  echo "bios: ${bios}"
    :  mkdir -p "\$path_bios"
    :  cp "$RUNNER_MOUNTPOINT/app/bios/$bios" "\$path_bios"
    :fi
    :
	END

  if [[ "$GIMG_PKG_TYPE" = flatimage ]]; then
    { sed -E 's/^\s+://' | tee -a AppDir/AppRun | sed -e 's/^/-- /'; } <<-END
      :export GIMG_LAUNCHER_NAME="$name"
      :export GIMG_LAUNCHER_IMG="$RUNNER_LAUNCHER_IMG"
      :launcher
      :
		END
  fi

  { sed -E 's/^\s+://' | tee -a AppDir/AppRun | sed -e 's/^/-- /'; } <<-END
    :if [[ "\$*" = "--config" ]]; then
    :  "$RUNNER_BIN"
    :elif [[ "\$*" ]]; then
    :  "$RUNNER_BIN" "\$@"
    :else
    :  "$RUNNER_BIN" -L "$RUNNER_MOUNTPOINT/app/core/${core}" "$RUNNER_MOUNTPOINT/app/rom/${rom}"
    :fi
	END

  # Allow execute
  chmod +x AppDir/AppRun
}

function main()
{
  build_emu "retroarch" "$@"
}

main "$@"
