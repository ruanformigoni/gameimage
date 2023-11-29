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

  if [[ "$GIMG_PKG_TYPE" = "flatimage" ]]; then
    url="$("$GIMG_SCRIPT_DIR"/busybox wget -q --header="Accept: application/vnd.github+json" -O - \
      https://api.github.com/repos/flatimage/flatimage-pcsx2/releases/latest 2>&1 |
      jq -r ".assets.[0].browser_download_url")"
  else
    url="$("$GIMG_SCRIPT_DIR"/busybox wget --header="Accept: application/vnd.github+json" -O - \
      https://api.github.com/repos/PCSX2/pcsx2/releases 2>&1 |
      grep -o "https://.*\.AppImage" | sort -V | tail -n1)"
  fi

  msg "pcsx2: ${url}"

  # Get pcsx2
  if [ ! -f "AppDir/usr/bin/pcsx2" ]; then
    _fetch "AppDir/usr/bin/pcsx2" "$url"
  fi
}

function runner_create_appimage()
{
  local bios="$(basename "$1")"
  local rom="$(basename "$3")"

  [ "$bios" == "null" ] && local bios=""
  [ "$rom" == "null" ] && { msg "Invalid rom file"; die; }

  # Create runner script
  { sed -E 's/^\s+://' | tee AppDir/AppRun | sed -e 's/^/-- /'; } <<-END
    :#!/usr/bin/env bash
    :
    :set -e
    :
    :# Platform
    :export GIMG_PLATFORM=$GIMG_PLATFORM
    :
    :# Package Type
    :export GIMG_PKG_TYPE=$GIMG_PKG_TYPE
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

function runner_create_flatimage()
{
  local bios="$(basename "$1")"
  local rom="$(basename "$3")"

  [ "$bios" == "null" ] && local bios=""
  [ "$rom" == "null" ] && { msg "Invalid rom file"; die; }

  # Create runner script
  { sed -E 's/^\s+://' | tee AppDir/app/gameimage.sh | sed -e 's/^/-- /'; } <<-END
    :#!/usr/bin/env bash
    :
    :set -e
    :
    :# Platform
    :export GIMG_PLATFORM=$GIMG_PLATFORM
    :
    :# Package Type
    :export GIMG_PKG_TYPE=$GIMG_PKG_TYPE
    :
    :# Path to pcsx2
    :export PATH="/pcsx2/bin:\$PATH"
    :
    :# Set cfg dir
    :export XDG_CONFIG_HOME="\${FIM_DIR_BINARY}/.\${FIM_FILE_BINARY}.config"
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
    :  cp "/app/bios/${bios}" "\${bios_path}/${bios}"
    :fi
    :
    :if [[ "\$*" = "--config" ]]; then
    :  pcsx2-qt
    :elif [[ "\$*" ]]; then
    :  pcsx2-qt "\$@"
    :else
    :  pcsx2-qt -- "\$APPDIR/app/rom/$rom"
    :fi
	END

  # Allow execute
  chmod +x AppDir/app/gameimage.sh
}

function build_flatimage()
{
  local name="$1"
  local bin_pkg="$BUILD_DIR"/pcsx2

  # Copy vanilla pcsx2 package
  cp "$BUILD_DIR/AppDir/usr/bin/pcsx2" "$bin_pkg"

  # Compress game dir
  "$bin_pkg" fim-exec mkdwarfs -f -i "$BUILD_DIR"/AppDir/app -o "$BUILD_DIR"/app.dwarfs

  # Include inside image
  "$bin_pkg" fim-include-path "$BUILD_DIR"/app.dwarfs /app.dwarfs

  # Default command -> runner script
  "$bin_pkg" fim-cmd /app/gameimage.sh

  # Copy cover
  "$bin_pkg" fim-exec mkdir -p /fim/desktop-integration
  "$bin_pkg" fim-exec cp "$BUILD_DIR/AppDir/${name}.png" /fim/desktop-integration/icon.png

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

  if [[ "$GIMG_PKG_TYPE" = "flatimage" ]]; then
    runner_create_flatimage "$bios" "$core" "$rom"
    build_flatimage "$name"
  else
    runner_create_appimage "$bios" "$core" "$rom"
    desktop_entry_create "$name"
    build_appimage
  fi
}

main "$@"
