#!/usr/bin/env bash

######################################################################
# @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
# @file        : yuzu
# @created     : Monday Sep 19, 2022 20:24:27 -03
######################################################################

#shellcheck disable=2155
#shellcheck source=/dev/null

set -e

shopt -s globstar

GIMG_SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

source "$GIMG_SCRIPT_DIR/common.sh"

function yuzu_download()
{
  local url

  if [[ "$GIMG_PKG_TYPE" = "flatimage" ]]; then
    url="$(_fetch_stdout https://api.github.com/repos/flatimage/flatimage-yuzu/releases/latest |
      jq -r ".assets.[0].browser_download_url")"
  else
    url="$(_fetch_stdout https://api.github.com/repos/yuzu-emu/yuzu-mainline/releases |
      jq -r ".[0].assets.[3].browser_download_url")"
  fi

  msg "yuzu: ${url}"

  # Get yuzu
  if [ ! -f "AppDir/usr/bin/yuzu" ]; then
    _fetch "./AppDir/usr/bin/yuzu" "$url"
  fi
}

function runner_create()
{
  local name="$1"
  local bios="$(basename "$2")"
  local keys="$(basename "$3")"
  local rom="$(basename "$4")"

  msg "Install the updates and DLC to pack into the AppImage (not the game itself)"

  # Define common variables for each package type
  # shellcheck disable=2016
  if [[ "$GIMG_PKG_TYPE" = "flatimage" ]]; then
    RUNNER_PATH='/fim/shared:/yuzu/bin:$PATH'
    RUNNER_XDG_CONFIG_HOME='${FIM_DIR_BINARY}/.${FIM_FILE_BINARY}.config/overlays/app/mount/xdg/config'
    RUNNER_XDG_DATA_HOME='${FIM_DIR_BINARY}/.${FIM_FILE_BINARY}.config/overlays/app/mount/xdg/data'
    RUNNER_MOUNTPOINT='$FIM_DIR_MOUNT'
    RUNNER_BIN='/fim/scripts/yuzu.sh'
    RUNNER_LAUNCHER_IMG='$FIM_DIR_MOUNT/fim/desktop-integration/icon.png'
  else
    RUNNER_PATH='$APPDIR/usr/bin:$PATH'
    RUNNER_XDG_CONFIG_HOME='$(dirname "$APPIMAGE")/.$(basename "$APPIMAGE").config/xdg/config'
    RUNNER_XDG_DATA_HOME='$(dirname "$APPIMAGE")/.$(basename "$APPIMAGE").config/xdg/data'
    RUNNER_MOUNTPOINT='$APPDIR'
    RUNNER_BIN='$APPDIR/usr/bin/yuzu'
  fi

  # Define & create temporary directories for builder
  BUILDER_XDG_CONFIG_HOME="$(pwd)"/AppDir/app/xdg/config
  BUILDER_XDG_DATA_HOME="$(pwd)"/AppDir/app/xdg/data
  mkdir -p "$BUILDER_XDG_DATA_HOME"/yuzu/nand/system/Contents/registered
  mkdir -p "$BUILDER_XDG_DATA_HOME"/yuzu/keys

  # Copy data such as bios and keys
  cp AppDir/app/bios/* "$BUILDER_XDG_DATA_HOME"/yuzu/nand/system/Contents/registered
  cp AppDir/app/keys/* "$BUILDER_XDG_DATA_HOME"/yuzu/keys

  (
    #shellcheck disable=2030
    export XDG_CONFIG_HOME="$BUILDER_XDG_CONFIG_HOME"
    #shellcheck disable=2030
    export XDG_DATA_HOME="$BUILDER_XDG_DATA_HOME"
    AppDir/usr/bin/yuzu
  )

  local bios="${bios#null}"

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
    :# Platform
    :export GIMG_PLATFORM=$GIMG_PLATFORM
    :echo "GIMG_PLATFORM: \${GIMG_PLATFORM}"
    :
    :# Package Type
    :export GIMG_PKG_TYPE=$GIMG_PKG_TYPE
    :echo "GIMG_PKG_TYPE: \${GIMG_PKG_TYPE}"
    :
    :# Path to yuzu
    :export PATH="$RUNNER_PATH"
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
    :# Force re-generate paths, they are incorrect
    :# after the image is created
    :rm -f "$RUNNER_XDG_CONFIG_HOME/yuzu/qt-config.ini"
    :rm -f "$RUNNER_XDG_CONFIG_HOME/QtProject.conf"
	END

  # For AppImage, copy files, no overlayfs yet
  if [[ "$GIMG_PKG_TYPE" = appimage ]]; then
  { sed -E 's/^\s+://' | tee -a AppDir/AppRun | sed -e 's/^/-- /'; } <<-END
    :if ! find "$RUNNER_XDG_DATA_HOME/yuzu/nand/system/Contents/registered" -iname "*.nca" -print -quit &>/dev/null || \\
    :   ! find "$RUNNER_XDG_DATA_HOME/yuzu/keys" -iname "*.keys" -print -quit &>/dev/null; then
    :  mkdir -p "$RUNNER_XDG_CONFIG_HOME"
    :  mkdir -p "$RUNNER_XDG_DATA_HOME"
    :  cp -r "$RUNNER_MOUNTPOINT"/app/xdg/config/* "$RUNNER_XDG_CONFIG_HOME"
    :  cp -r "$RUNNER_MOUNTPOINT"/app/xdg/data/* "$RUNNER_XDG_DATA_HOME"
    :fi
		END
  fi

  # Set launcher options
  if [[ "$GIMG_PKG_TYPE" = flatimage ]]; then
    { sed -E 's/^\s+://' | tee -a AppDir/AppRun | sed -e 's/^/-- /'; } <<-END
      :export GIMG_LAUNCHER_NAME="$name"
      :export GIMG_LAUNCHER_IMG="$RUNNER_LAUNCHER_IMG"
      :gui-launcher
      :
		END
  fi

  { sed -E 's/^\s+://' | tee -a AppDir/AppRun | sed -e 's/^/-- /'; } <<-END
    :if [[ "\$*" = "--config" ]]; then
    :  "$RUNNER_BIN"
    :elif [[ "\$*" ]]; then
    :  "$RUNNER_BIN" "\$@"
    :else
    :  "$RUNNER_BIN" -f -g "$RUNNER_MOUNTPOINT/app/rom/${rom}"
    :fi
	END

  # Allow execute
  chmod +x AppDir/AppRun
}

function build_flatimage()
{
  local name="$1"
  local bin_pkg="$BUILD_DIR"/yuzu

  # Copy vanilla yuzu package
  cp "$BUILD_DIR/AppDir/usr/bin/yuzu" "$bin_pkg"

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
  "$bin_pkg" fim-root cp "${GIMG_SCRIPT_DIR}/gui-launcher" '/fim/shared/gui-launcher'

  # Set HOME dir
  # shellcheck disable=2016
  "$bin_pkg" fim-config-set home '$FIM_DIR_BINARY/.${FIM_FILE_BINARY}.config'

  # Rename binary
  mv "$bin_pkg" "$BUILD_DIR/${name}.fim"
}

function main()
{
  # Validate params
  params_validate "yuzu" "$@"

  local name="${_FN_RET[0]}"
  local dir="${_FN_RET[1]}"
  local bios="${_FN_RET[2]}"
  local cover="${_FN_RET[4]}"
  local rom="${_FN_RET[5]}"
  local keys="${_FN_RET[6]}"

  # Create dirs
  cd "$(dir_build_create "$dir")"

  BUILD_DIR="$(pwd)"

  dir_appdir_create

  # Download tools
  if [[ "$GIMG_PKG_TYPE" = "appimage" ]]; then
    _fetch_appimagetool
  fi
  yuzu_download
  _fetch_imagemagick

  # Populate appdir
  files_copy "$name" "$dir" "$bios" "null" "$cover" "$keys"

  runner_create "$name" "$bios" "$keys" "$rom"

  if [[ "$GIMG_PKG_TYPE" = "flatimage" ]]; then
    build_flatimage "$name"
  else
    desktop_entry_create "$name"
    build_appimage
  fi

}

main "$@"
