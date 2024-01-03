#!/tmp/gameimage/bin/bash

######################################################################
# @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
# @file        : test
# @created     : Monday Sep 19, 2022 20:24:27 -03
######################################################################

#shellcheck disable=2155
#shellcheck source=/dev/null

set -e

shopt -s globstar

GIMG_SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

source "$GIMG_SCRIPT_DIR/common.sh"


function rpcs3_download()
{
  local url

  if [[ "$GIMG_PKG_TYPE" = "flatimage" ]]; then
    url="$(_fetch_stdout "https://api.github.com/repos/flatimage/flatimage-rpcs3/releases/latest" \
      | jq -r ".assets.[0].browser_download_url")"
  else
    url="$(_fetch_stdout "https://api.github.com/repos/RPCS3/rpcs3-binaries-linux/releases/latest" \
      | jq -r ".assets.[0].browser_download_url")"
  fi

  msg "rpcs3: ${url}"

  # Get rpcs3
  if [ ! -f "AppDir/usr/bin/rpcs3" ]; then
    _fetch "AppDir/usr/bin/rpcs3" "$url"
    if [[ "$GIMG_PKG_TYPE" = "flatimage" ]]; then
      # Set home directory to build dir
      ./AppDir/usr/bin/rpcs3 fim-config-set home "$DIR_SRC"
    fi
  fi
}

function runner_create()
{
  local name="$1"
  local bios="$(basename "$2")"

  # Define common variables for each package type
  # shellcheck disable=2016
  if [[ "$GIMG_PKG_TYPE" = "flatimage" ]]; then
    RUNNER_PATH='/fim/shared:/rpcs3/bin:$PATH'
    RUNNER_XDG_CONFIG_HOME='${FIM_DIR_BINARY}/.${FIM_FILE_BINARY}.config/overlays/app/mount/xdg/config'
    RUNNER_XDG_DATA_HOME='${FIM_DIR_BINARY}/.${FIM_FILE_BINARY}.config/overlays/app/mount/xdg/data'
    RUNNER_MOUNTPOINT='$FIM_DIR_MOUNT'
    RUNNER_BIN='/fim/scripts/rpcs3.sh'
    RUNNER_LAUNCHER_IMG='$FIM_DIR_MOUNT/fim/desktop/icon.png'
  else
    RUNNER_PATH='$APPDIR/usr/bin:$PATH'
    RUNNER_XDG_CONFIG_HOME='$(dirname "$APPIMAGE")/.$(basename "$APPIMAGE").config/xdg/config'
    RUNNER_XDG_DATA_HOME='$(dirname "$APPIMAGE")/.$(basename "$APPIMAGE").config/xdg/data'
    RUNNER_MOUNTPOINT='$APPDIR'
    RUNNER_BIN='$APPDIR/usr/bin/rpcs3'
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
	END

  if [[ "$GIMG_PKG_TYPE" = flatimage ]]; then
    { sed -E 's/^\s+://' | tee -a AppDir/AppRun | sed -e 's/^/-- /'; } <<-END
      :export GIMG_LAUNCHER_NAME="$name"
      :export GIMG_LAUNCHER_IMG="$RUNNER_LAUNCHER_IMG"
      :launcher
      :
      :# Workdaround for 'cannot start from temporary location'
      :rm -f "\${FIM_DIR_BINARY}/.\${FIM_FILE_BINARY}.config/overlays/app/mount"
		END
  fi

  { sed -E 's/^\s+://' | tee -a AppDir/AppRun | sed -e 's/^/-- /'; } <<-END
    :echo "bios: ${bios}"
    :
    :# Check if bios is installed
    :if ! find "\${XDG_CONFIG_HOME}/rpcs3/dev_flash/sys/internal" -iname "*.sprx" -print -quit &>/dev/null; then
    :  "$RUNNER_BIN" --installfw "$RUNNER_MOUNTPOINT/app/bios/${bios}"
    :fi
    :
    :if [[ "\$*" = "--config" ]]; then
    :  "$RUNNER_BIN"
    :elif [[ "\$*" ]]; then
    :  "$RUNNER_BIN" "\$@"
    :else
    :  "$RUNNER_BIN" --no-gui "$RUNNER_MOUNTPOINT/app/rom"
    :fi
    :
	END

  # Allow executable
  chmod +x AppDir/AppRun
}

function main()
{
  # Validate params
  params_validate "rpcs3" "$@"

  local name="${_FN_RET[0]}"
  local dir="${_FN_RET[1]}"
  local bios="${_FN_RET[2]}"
  local core="${_FN_RET[3]}"
  local cover="${_FN_RET[4]}"

  # Export dir src
  export DIR_SRC="$dir"

  # Create dirs
  cd "$(dir_build_create "$dir")"

  export DIR_BUILD="$(pwd)"

  dir_appdir_create

  # Download tools
  if [[ "$GIMG_PKG_TYPE" = "appimage" ]]; then
    _fetch_appimagetool
  fi
  rpcs3_download
  _fetch_imagemagick

  # Populate appdir
  files_copy "$name" "$dir" "$bios" "$core" "$cover" "null"

  runner_create "$name" "$bios"

  if [[ "$GIMG_PKG_TYPE" = "flatimage" ]]; then
    build_flatimage_emu "$name" "rpcs3"
  else
    desktop_entry_create "$name"
    build_appimage
  fi
}

main "$@"
