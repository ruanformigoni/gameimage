#!/usr/bin/env bash

######################################################################
# @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
# @file        : test
# @created     : Monday Sep 19, 2022 20:24:27 -03
######################################################################

#shellcheck disable=2155
#shellcheck source=/dev/null

set -e

shopt -s globstar

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

source "$SCRIPT_DIR/common.sh"


function rpcs3_download()
{
  # Get rpcs3
  if [ ! -f "AppDir/usr/bin/rpcs3" ]; then
    if [ ! -f "rpcs3.AppImage" ]; then
      # Get AppImage of rpcs3
      wget -q --show-progress --progress=bar:noscroll -O rpcs3.AppImage "https://github.com/RPCS3/rpcs3-binaries-linux/releases/download/build-67c02e3522a576d1d739fa130f484ab9a64b5d62/rpcs3-v0.0.24-14195-67c02e35_linux64.AppImage"

      # Make executable
      chmod +x ./rpcs3.AppImage
    fi

    # Move to AppDir
    cp rpcs3.AppImage AppDir/usr/bin/rpcs3
  fi
}

function runner_create()
{
  local bios="$(basename "$1")"

  # Create runner script
  { sed -E 's/^\s+://' | tee AppDir/AppRun; } <<-END
    :#!/usr/bin/env bash
    :
    :set -e
    :
    :# Check if config dir is set
    :[ -n "\${XDG_CONFIG_HOME}" ] || XDG_CONFIG_HOME="\$HOME/.config"
    :
    :# Check if bios is installed
    :if ! find "\${XDG_CONFIG_HOME}/rpcs3/dev_flash/sys/internal" -iname "*.sprx" -print -quit &>/dev/null; then
    :  "\$APPDIR/usr/bin/rpcs3" --installfw "\$APPDIR/app/${bios}"
    :fi
    :
    :echo "XDG_CONFIG_HOME: \${XDG_CONFIG_HOME}"
    :echo "bios: ${bios}"
    :
    :"\$APPDIR/usr/bin/rpcs3" --no-gui "\$APPDIR/app"
	END

  # Allow executable
  chmod +x AppDir/AppRun
}

function appimagebuilder_download()
{
  # Get appimagebuilder
  [ ! -f "./appimagebuilder.AppImage" ] && wget -q --show-progress --progress=bar:noscroll -O appimagebuilder.AppImage "https://github.com/AppImageCrafters/appimage-builder/releases/download/v1.1.0/appimage-builder-1.1.0-x86_64.AppImage"

  chmod +x ./appimagebuilder.AppImage
}

function main()
{
  # Validate params
  readarray -t ret <<< "$(params_validate "$@")"

  local name="${ret[0]}"
  local dir="${ret[1]}"
  local bios="${ret[2]}"
  local core="${ret[3]}"
  local cover="${ret[4]}"

  # Create dirs
  build_dir="$(dir_build_create)"; cd "${build_dir}"

  dir_appdir_create

  # Download tools
  appimagetool_download

  rpcs3_download

  # Populate appdir
  files_copy "$name" "$dir" "$bios" "$core" "$cover"

  runner_create "$bios"

  desktop_entry_create "$name"

  # Build appimage
  appdir_build
}

main "$@"
