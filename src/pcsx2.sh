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
    RUNNER_LAUNCHER_IMG='$FIM_DIR_MOUNT/fim/desktop/icon.png'
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

function main()
{
  build_emu "pcsx2" "$@"
}

main "$@"
