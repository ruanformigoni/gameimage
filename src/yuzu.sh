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

  url="$(curl -H "Accept: application/vnd.github+json" \
    https://api.github.com/repos/yuzu-emu/yuzu-mainline/releases 2>&1 |
    grep -o "https://.*\.AppImage" | sort | tail -n1)"

  msg "yuzu: ${url}"

  # Get yuzu
  if [ ! -f "AppDir/usr/bin/yuzu" ]; then
    if [ ! -f "yuzu.AppImage" ]; then
      # Get AppImage of yuzu
      if [ "$GIMG_YAML" ]; then
        wget -q --show-progress --progress=dot:mega -O yuzu "$url"
      else
        wget -q --show-progress --progress=bar:noscroll -O yuzu "$url"
      fi
      # Make executable
      chmod +x ./yuzu
    fi

    # Move to AppDir
    cp yuzu AppDir/usr/bin/yuzu
  fi
}

function runner_create()
{
  local bios="$(basename "$1")"
  local keys="$(basename "$2")"
  local rom="$(basename "$3")"

  msg "Install the updates and DLC to pack into the AppImage (not the game itself)"

  mkdir -p AppDir/app/config/home

  mkdir -p AppDir/app/config/home/.local/share/yuzu/nand/system/Contents/registered
  mkdir -p AppDir/app/config/home/.local/share/yuzu/keys

  cp AppDir/app/bios/* AppDir/app/config/home/.local/share/yuzu/nand/system/Contents/registered
  cp AppDir/app/keys/* AppDir/app/config/home/.local/share/yuzu/keys

  echo "$(export XDG_CONFIG_HOME="$(pwd)"/AppDir/app/config; \
    export HOME="$(pwd)"/AppDir/app/config/home; \
    AppDir/usr/bin/yuzu)"

  [ "$bios" == "null" ] && local bios=""

  # Create runner script
  { sed -E 's/^\s+://' | tee AppDir/AppRun | sed -e 's/^/-- /'; } <<-END
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
    :  export HOME="\$XDG_CONFIG_HOME/home"
    :fi
    :
    :echo "XDG_CONFIG_HOME: \${XDG_CONFIG_HOME}"
    :
    :if ! find "\$HOME/.local/share/yuzu/nand/system/Contents/registered" -iname "*.nca" -print -quit &>/dev/null || \\
    :   ! find "\$HOME/.local/share/yuzu/keys" -iname "*.keys" -print -quit &>/dev/null; then
    :  mkdir -p "\$HOME"
    :  cp -r "\$APPDIR"/app/config/* "\${XDG_CONFIG_HOME}"
    :fi
    :
    :if [[ "\$*" = "--config" ]]; then
    :  "\$APPDIR/usr/bin/yuzu"
    :elif [[ "\$*" ]]; then
    :  "\$APPDIR/usr/bin/yuzu" "\$@"
    :else
    :  "\$APPDIR/usr/bin/yuzu" -f -g "\$APPDIR/app/rom/${rom}"
    :fi
	END

  # Allow execute
  chmod +x AppDir/AppRun
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

  dir_appdir_create

  # Download tools
  _fetch_appimagetool
  yuzu_download
  _fetch_imagemagick

  # Populate appdir
  files_copy "$name" "$dir" "$bios" "null" "$cover" "$keys"

  runner_create "$bios" "$keys" "$rom"

  desktop_entry_create "$name"

  # Build appimage
  appdir_build
}

main "$@"
