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

GIMG_SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

source "$GIMG_SCRIPT_DIR/common.sh"

function retroarch_download()
{
  local url

  # Get retroarch url
  if [[ "$GIMG_PKG_TYPE" = "flatimage" ]]; then
    url="$("$GIMG_SCRIPT_DIR"/busybox wget -q --header="Accept: application/vnd.github+json" -O - \
      https://api.github.com/repos/flatimage/flatimage-retroarch/releases/latest 2>&1 |
      jq -r ".assets.[0].browser_download_url")"
  else
    url="https://buildbot.libretro.com/nightly/linux/x86_64/RetroArch.7z"
  fi

  if [ ! -f "AppDir/usr/bin/retroarch" ]; then
    msg "retroarch: $url"

    if [[ "$GIMG_PKG_TYPE" = "flatimage" ]]; then
      _fetch "AppDir/usr/bin/retroarch" "$url"
    else
      # AppImage requires additional patching to avoid crashes

      # Get AppImage of retroarch
      "$GIMG_SCRIPT_DIR"/busybox wget "$url"

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
  local core="$(basename "$3")"
  local rom="$(basename "$4")"

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
    export RUNNER_LAUNCHER_IMG='$FIM_DIR_MOUNT/fim/desktop-integration/icon.png'
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
      :gameimage-launcher
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

function build_flatimage()
{
  local name="$1"
  local bin_pkg="$BUILD_DIR"/retroarch

  # Copy vanilla retroarch package
  cp "$BUILD_DIR/AppDir/usr/bin/retroarch" "$bin_pkg"

  # Compress game dir
  "$bin_pkg" fim-exec mkdwarfs -f -i "$BUILD_DIR"/AppDir/app -o "$BUILD_DIR"/app.dwarfs

  # Include inside image
  "$bin_pkg" fim-include-path "$BUILD_DIR"/app.dwarfs /app.dwarfs

  # Configure /app overlay
  "$bin_pkg" fim-config-set overlay.app '/app Overlay'
  # shellcheck disable=2016
  "$bin_pkg" fim-config-set overlay.app.host '${FIM_DIR_BINARY}/.${FIM_FILE_BINARY}.config/overlays/app'
  "$bin_pkg" fim-config-set overlay.app.cont '/app'

  # Include launcher script
  "$bin_pkg" fim-root mkdir -p /fim/scripts
  "$bin_pkg" fim-root cp "$BUILD_DIR/AppDir/AppRun" /fim/scripts/gameimage.sh

  # Default command -> runner script
  "$bin_pkg" fim-cmd /fim/scripts/gameimage.sh

  # Copy cover
  "$bin_pkg" fim-exec mkdir -p /fim/desktop-integration
  "$bin_pkg" fim-exec cp "$BUILD_DIR/AppDir/${name}.png" /fim/desktop-integration/icon.png

  # Copy launcher
  # shellcheck disable=2016
  "$bin_pkg" fim-root mkdir -p /fim/shared
  "$bin_pkg" fim-root cp "${GIMG_SCRIPT_DIR}/launcher-shared" '/fim/shared/gameimage-launcher'

  # Set HOME dir
  # shellcheck disable=2016
  "$bin_pkg" fim-config-set home '$FIM_DIR_BINARY/.${FIM_FILE_BINARY}.config'

  # Rename binary
  mv "$bin_pkg" "$BUILD_DIR/${name}.fim"
}

function main()
{
  # Validate params
  params_validate "retroarch" "$@"

  local name="${_FN_RET[0]}"
  local dir="${_FN_RET[1]}"
  local bios="${_FN_RET[2]}"
  local core="${_FN_RET[3]}"
  local cover="${_FN_RET[4]}"
  local rom="${_FN_RET[5]}"

  # Create dirs
  cd "$(dir_build_create "$dir")"

  BUILD_DIR="$(pwd)"

  dir_appdir_create

  # Download tools
  if [[ "$GIMG_PKG_TYPE" = "appimage" ]]; then
    _fetch_appimagetool
  fi
  retroarch_download
  _fetch_imagemagick

  # Populate appdir
  files_copy "$name" "$dir" "$bios" "$core" "$cover" "null"

  runner_create "$name" "$bios" "$core" "$rom"

  # Create runner script and build image
  if [[ "$GIMG_PKG_TYPE" = "flatimage" ]]; then
    build_flatimage "$name"
  else
    desktop_entry_create "$name"
    build_appimage
  fi
}

main "$@"
