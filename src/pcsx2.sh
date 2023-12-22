#!/tmp/gameimage/bin/bash

######################################################################
# @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
# @file        : pcsx2
# @created     : Monday Sep 19, 2022 20:24:27 -03
######################################################################

#shellcheck disable=2155
#shellcheck source=/dev/null

set -e

shopt -s globstar

GIMG_SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

source "$GIMG_SCRIPT_DIR/common.sh"

function pcsx2_download()
{
  local url

  if [[ "$GIMG_PKG_TYPE" = "flatimage" ]]; then
    url="$(_fetch_stdout "https://api.github.com/repos/flatimage/flatimage-pcsx2/releases/latest" |
      jq -r ".assets.[0].browser_download_url")"
  else
    url="$(_fetch_stdout "https://api.github.com/repos/PCSX2/pcsx2/releases" \
      | jq -r ".[0].assets.[0].browser_download_url")"
  fi

  msg "pcsx2: ${url}"

  # Get pcsx2
  if [ ! -f "AppDir/usr/bin/pcsx2" ]; then
    _fetch "AppDir/usr/bin/pcsx2" "$url"
    if [[ "$GIMG_PKG_TYPE" = "flatimage" ]]; then
      # Set home directory to build dir
      ./AppDir/usr/bin/pcsx2 fim-config-set home "$DIR_SRC"
    fi
  fi
}

function runner_create()
{
  local name="$1"; shift
  local bios="$(basename "$1")"; shift
  local rom="$(basename "$1")"; shift

  [ "$bios" == "null" ] && local bios=""
  [ "$rom" == "null" ] && { msg "Invalid rom file"; die; }

  # Define common variables for each package type
  # shellcheck disable=2016
  if [[ "$GIMG_PKG_TYPE" = "flatimage" ]]; then
    RUNNER_PATH='/fim/shared:/pcsx2/bin:$PATH'
    RUNNER_XDG_CONFIG_HOME='${FIM_DIR_BINARY}/.${FIM_FILE_BINARY}.config/overlays/app/mount/xdg/config'
    RUNNER_XDG_DATA_HOME='${FIM_DIR_BINARY}/.${FIM_FILE_BINARY}.config/overlays/app/mount/xdg/data'
    RUNNER_MOUNTPOINT='$FIM_DIR_MOUNT'
    RUNNER_BIN='/fim/scripts/pcsx2.sh'
    RUNNER_LAUNCHER_IMG='$FIM_DIR_MOUNT/fim/desktop-integration/icon.png'
  else
    RUNNER_PATH='$APPDIR/usr/bin:$PATH'
    RUNNER_XDG_CONFIG_HOME='$(dirname "$APPIMAGE")/.$(basename "$APPIMAGE").config/xdg/config'
    RUNNER_XDG_DATA_HOME='$(dirname "$APPIMAGE")/.$(basename "$APPIMAGE").config/xdg/data'
    RUNNER_MOUNTPOINT='$APPDIR'
    RUNNER_BIN='$APPDIR/usr/bin/pcsx2'
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
    :# Bios path
    :bios_path="\${XDG_CONFIG_HOME}/PCSX2/bios"
    :
    :# Create path if not exists
    :mkdir -p "\${bios_path}"
    :
    :echo "bios: ${bios}"
    :echo "bios_path: \${bios_path}"
    :
    :if [ ! -f "\${bios_path}/${bios}" ]; then
    :  cp "$RUNNER_MOUNTPOINT/app/bios/${bios}" "\${bios_path}/${bios}"
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
    :  "$RUNNER_BIN" -- "$RUNNER_MOUNTPOINT/app/rom/$rom"
    :fi
	END

  # Allow execute
  chmod +x AppDir/AppRun
}

function build_flatimage()
{
  local name="$1"
  local bin_pkg="$BUILD_DIR"/pcsx2

  # Copy vanilla pcsx2 package
  cp "$BUILD_DIR/AppDir/usr/bin/pcsx2" "$bin_pkg"

  # Compress game dir
  "$bin_pkg" fim-exec mkdwarfs -f \
    -i "$BUILD_DIR"/AppDir/app \
    -o "$BUILD_DIR"/app.dwarfs \
    -l"$GIMG_COMPRESSION_LEVEL"

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
  "$bin_pkg" fim-root cp "${GIMG_SCRIPT_DIR}/launcher" '/fim/shared/launcher'

  # Set HOME dir
  # shellcheck disable=2016
  "$bin_pkg" fim-config-set home '$FIM_DIR_BINARY/.${FIM_FILE_BINARY}.config'

  # Rename binary
  mv "$bin_pkg" "$BUILD_DIR/${name}.fim"
}

function main()
{
  # Validate params
  params_validate "pcsx2" "$@"

  local name="${_FN_RET[0]}"
  local dir="${_FN_RET[1]}"
  local bios="${_FN_RET[2]}"
  local core="${_FN_RET[3]}"
  local cover="${_FN_RET[4]}"
  local rom="${_FN_RET[5]}"

  # Export dir src
  export DIR_SRC="$dir"

  # Create dirs
  cd "$(dir_build_create "$dir")"

  BUILD_DIR="$(pwd)"

  dir_appdir_create

  # Download tools
  if [[ "$GIMG_PKG_TYPE" = "appimage" ]]; then
    _fetch_appimagetool
  fi
  pcsx2_download
  _fetch_imagemagick

  # Populate appdir
  files_copy "$name" "$dir" "$bios" "$core" "$cover" "null"

  runner_create "$name" "$bios" "$rom"

  if [[ "$GIMG_PKG_TYPE" = "flatimage" ]]; then
    build_flatimage "$name"
  else
    desktop_entry_create "$name"
    build_appimage
  fi
}

main "$@"
