#!/usr/bin/env bash

######################################################################
# @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
# @file        : pcsx2
# @created     : Monday Sep 19, 2022 20:24:27 -03
######################################################################

#shellcheck disable=2155
#shellcheck source=/dev/null

set -e

shopt -s globstar

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

source "$SCRIPT_DIR/common.sh"

function pcsx2_download()
{
  # Get pcsx2
  if [ ! -f "AppDir/usr/bin/pcsx2" ]; then
    if [ ! -f "pcsx2.AppImage" ]; then
      # Get AppImage of pcsx2
      wget -q --show-progress --progress=bar:noscroll -O pcsx2.AppImage "https://github.com/PCSX2/pcsx2/releases/download/v1.7.3339/pcsx2-v1.7.3339-linux-AppImage-64bit-AVX2-Qt.AppImage"

      # Make executable
      chmod +x ./pcsx2.AppImage
    fi

    # Move to AppDir
    cp pcsx2.AppImage AppDir/usr/bin/pcsx2
  fi
}

function runner_create()
{
  local bios="$(basename "$1")"
  local rom="$(basename "$3")"

  [ "$bios" == "null" ] && local bios=""

  # Create runner script
  { sed -E 's/^\s+://' | tee AppDir/AppRun; } <<-END
    :#!/usr/bin/env bash
    :
    :set -e
    :
    :# Check if config dir is set
    :[ -n "\${XDG_CONFIG_HOME}" ] || XDG_CONFIG_HOME="\$HOME/.config"
    :
    :# Bios path
    :bios_path="\${XDG_CONFIG_HOME}/PCSX2/bios"
    :
    :# Create path if not exists
    :mkdir -p \${bios_path}
    :
    :echo "XDG_CONFIG_HOME: \${XDG_CONFIG_HOME}"
    :echo "bios: ${bios}"
    :echo "bios_path: \${bios_path}"
    :
    :if [ ! -f "\${bios_path}/${bios}" ]; then
    :  cp "\$APPDIR/app/${bios}" "\${bios_path}/${bios}"
    :fi
    :
    :"\$APPDIR/usr/bin/pcsx2" -- "\$APPDIR/app/$rom"
	END

  # Allow execute
  chmod +x AppDir/AppRun
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
  local rom="${ret[5]}"

  # Create dirs
  build_dir="$(dir_build_create)"; cd "${build_dir}"

  dir_appdir_create

  # Download tools
  appimagetool_download

  pcsx2_download

  # Populate appdir
  files_copy "$name" "$dir" "$bios" "$core" "$cover"

  runner_create "$bios" "$core" "$rom"

  desktop_entry_create "$name"

  # Build appimage
  appdir_build
}

main "$@"
