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
  # Get retroarch
  if [ ! -f "AppDir/usr/bin/retroarch" ]; then
    if [ ! -f "RetroArch-x86_64.AppImage" ]; then
      local url

      url="https://buildbot.libretro.com/nightly/linux/x86_64/RetroArch.7z"

      msg "retroarch: $url"

      # Get AppImage of retroarch
      if [ "$GIMG_YAML" ]; then
        wget -q --show-progress --progress=dot:giga "$url"
      else
        wget -q --show-progress --progress=bar:noscroll "$url"
      fi

      # Extract and move
      7z x "RetroArch.7z"
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
    fi

    # Copy assets into the AppImage
    cp -r config AppDir/app/config 

    # Copy retroarch into the appimage
    cp RetroArch*.AppImage AppDir/usr/bin/retroarch
  fi
}

function runner_create()
{
  local bios="$(basename "$1")"
  local core="$(basename "$2")"
  local rom="$(basename "$3")"

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
    :fi
    :
    :echo "XDG_CONFIG_HOME: \${XDG_CONFIG_HOME}"
    :
    :# Check if retroarch assets are missing
    :dir_retroarch_assets="\$XDG_CONFIG_HOME/retroarch"
    :if [ ! -d "\$dir_retroarch_assets" ]; then
    :  mkdir -p "\$dir_retroarch_assets"
    :  cp -r "\$APPDIR"/app/config/retroarch/* "\$dir_retroarch_assets"
    :fi
    :
    :path_bios=\$XDG_CONFIG_HOME/retroarch/system/
    :
    :if [ "$bios" ] && [ ! -f "\${path_bios}/$bios" ]; then
    :  echo "bios: ${bios}"
    :  mkdir -p "\$path_bios"
    :  cp "\$APPDIR/app/bios/$bios" "\$path_bios"
    :fi
    :
    :if [[ "\$*" = "--config" ]]; then
    :  "\$APPDIR/usr/bin/retroarch"
    :elif [[ "\$*" ]]; then
    :  "\$APPDIR/usr/bin/retroarch" "\$@"
    :else
    :  "\$APPDIR/usr/bin/retroarch" -L "\$APPDIR/app/core/${core}" "\$APPDIR/app/rom/${rom}"
    :fi
	END

  # Allow execute
  chmod +x AppDir/AppRun
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

  dir_appdir_create

  # Download tools
  _fetch_appimagetool
  retroarch_download
  _fetch_imagemagick

  # Populate appdir
  files_copy "$name" "$dir" "$bios" "$core" "$cover" "null"

  runner_create "$bios" "$core" "$rom"

  desktop_entry_create "$name"

  # Build appimage
  appdir_build
}

main "$@"
