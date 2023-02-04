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

GIMG_SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

source "$GIMG_SCRIPT_DIR/common.sh"

function pcsx2_download()
{
  local url

  url="$(curl -H "Accept: application/vnd.github+json" https://api.github.com/repos/PCSX2/pcsx2/releases 2>&1 |
    grep -o "https://.*\.AppImage" | sort -V | tail -n1)"

  msg "pcsx2: ${url}"

  # Get pcsx2
  if [ ! -f "AppDir/usr/bin/pcsx2" ]; then
    if [ ! -f "pcsx2.AppImage" ]; then
      # Get AppImage of pcsx2
      if [ "$GIMG_YAML" ]; then
        wget -q --show-progress --progress=dot:mega -O pcsx2.AppImage "$url"
      else
        wget -q --show-progress --progress=bar:noscroll -O pcsx2.AppImage "$url"
      fi

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
  [ "$rom" == "null" ] && { msg "Invalid rom file"; die; }

  # Create runner script
  { sed -E 's/^\s+://' | tee AppDir/AppRun; } <<-END
    :#!/usr/bin/env bash
    :
    :set -e
    :
    :# Set cfg dir
    :if [[ "\$(basename "\${APPIMAGE}")" =~ \.\.AppImage ]]; then
    :  # Set global
    : export XDG_CONFIG_HOME="\$HOME/.config"
    :else
    :  # Set local
    :  export XDG_CONFIG_HOME="\$(dirname "\$APPIMAGE")/.\$(basename "\$APPIMAGE").config"
    :fi
    :
    :# Bios path
    :bios_path="\${XDG_CONFIG_HOME}/PCSX2/bios"
    :
    :# Create path if not exists
    :mkdir -p "\${bios_path}"
    :
    :echo "XDG_CONFIG_HOME: \${XDG_CONFIG_HOME}"
    :echo "bios: ${bios}"
    :echo "bios_path: \${bios_path}"
    :
    :if [ ! -f "\${bios_path}/${bios}" ]; then
    :  cp "\$APPDIR/app/bios/${bios}" "\${bios_path}/${bios}"
    :fi
    :
    :if [[ "\$*" = "--config" ]]; then
    :  "\$APPDIR/usr/bin/pcsx2"
    :elif [[ "\$*" ]]; then
    :  "\$APPDIR/usr/bin/pcsx2" "\$@"
    :else
    :  "\$APPDIR/usr/bin/pcsx2" -- "\$APPDIR/app/rom/$rom"
    :fi
	END

  # Allow execute
  chmod +x AppDir/AppRun
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

  # Create dirs
  cd "$(dir_build_create "$dir")"

  dir_appdir_create

  # Download tools
  _fetch_appimagetool
  pcsx2_download
  _fetch_imagemagick

  # Populate appdir
  files_copy "$name" "$dir" "$bios" "$core" "$cover" "null"

  runner_create "$bios" "$core" "$rom"

  desktop_entry_create "$name"

  # Build appimage
  appdir_build
}

main "$@"
