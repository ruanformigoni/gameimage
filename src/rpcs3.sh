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

GIMG_SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

source "$GIMG_SCRIPT_DIR/common.sh"


function rpcs3_download()
{
  local url

  if [[ "$GIMG_PKG_TYPE" = "flatimage" ]]; then
    url="$("$GIMG_SCRIPT_DIR"/busybox wget -q --header="Accept: application/vnd.github+json" -O - \
      https://api.github.com/repos/flatimage/flatimage-rpcs3/releases/latest 2>&1 |
      jq -r ".assets.[0].browser_download_url")"
  else
    url="$("$GIMG_SCRIPT_DIR"/busybox wget -q --header="Accept: application/vnd.github+json" -O - \
      https://api.github.com/repos/RPCS3/rpcs3-binaries-linux/releases/latest 2>&1 |
      grep -o "https://.*\.AppImage")"
  fi

  msg "rpcs3: ${url}"

  # Get rpcs3
  if [ ! -f "AppDir/usr/bin/rpcs3" ]; then
    _fetch "AppDir/usr/bin/rpcs3" "$url"
  fi
}

function runner_create_appimage()
{
  local bios="$(basename "$1")"

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
    :  export XDG_CONFIG_HOME="\$HOME/.config"
    :else
    :  # Set local
    :  export XDG_CONFIG_HOME="\$(dirname "\$APPIMAGE")/.\$(basename "\$APPIMAGE").config"
    :fi
    :
    :mkdir -p "\$XDG_CONFIG_HOME"
    :
    :echo "XDG_CONFIG_HOME: \${XDG_CONFIG_HOME}"
    :echo "bios: ${bios}"
    :
    :# Check if bios is installed
    :if ! find "\${XDG_CONFIG_HOME}/rpcs3/dev_flash/sys/internal" -iname "*.sprx" -print -quit &>/dev/null; then
    :  "\$APPDIR/usr/bin/rpcs3" --installfw "\$APPDIR/app/bios/${bios}"
    :fi
    :
    :if [[ "\$*" = "--config" ]]; then
    :  "\$APPDIR/usr/bin/rpcs3"
    :elif [[ "\$*" ]]; then
    :  "\$APPDIR/usr/bin/rpcs3" "\$@"
    :else
    :  "\$APPDIR/usr/bin/rpcs3" --no-gui "\$APPDIR/app/rom"
    :fi
	END

  # Allow executable
  chmod +x AppDir/AppRun
}

function runner_create_flatimage()
{
  local bios="$(basename "$1")"

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
    :# Path to rpcs3
    :export PATH="/rpcs3/bin:\$PATH"
    :
    :# Set cfg dir
    :export XDG_CONFIG_HOME="\${FIM_DIR_BINARY}/.\${FIM_FILE_BINARY}.config"
    :mkdir -p "\$XDG_CONFIG_HOME"
    :
    :echo "XDG_CONFIG_HOME: \${XDG_CONFIG_HOME}"
    :echo "bios: ${bios}"
    :
    :# Check if bios is installed
    :if ! find "\${XDG_CONFIG_HOME}/rpcs3/dev_flash/sys/internal" -iname "*.sprx" -print -quit &>/dev/null; then
    :  rpcs3 --installfw "/app/bios/${bios}"
    :fi
    :
    :if [[ "\$*" = "--config" ]]; then
    :  rpcs3
    :elif [[ "\$*" ]]; then
    :  rpcs3 "\$@"
    :else
    :  rpcs3 --no-gui "/app/rom"
    :fi
	END

  # Allow executable
  chmod +x AppDir/app/gameimage.sh
}

function build_flatimage()
{
  local name="$1"
  local bin_pkg="$BUILD_DIR"/rpcs3

  # Copy vanilla rpcs3 package
  cp "$BUILD_DIR/AppDir/usr/bin/rpcs3" "$bin_pkg"

  # Compress game dir
  "$bin_pkg" fim-exec mkdwarfs -f -i "$BUILD_DIR"/AppDir/app -o "$BUILD_DIR"/app.dwarfs

  # Include inside image
  "$bin_pkg" fim-include-path "$BUILD_DIR"/app.dwarfs /app.dwarfs

  # Default command -> runner script
  "$bin_pkg" fim-cmd /app/gameimage.sh

  # Copy cover
  "$bin_pkg" fim-exec mkdir -p /fim/desktop-integration
  "$bin_pkg" fim-exec cp "$BUILD_DIR/AppDir/${name}.png" /fim/desktop-integration/icon.png

  # Rename binary
  mv "$bin_pkg" "$BUILD_DIR/${name}.fim"
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

  # Create dirs
  cd "$(dir_build_create "$dir")"

  BUILD_DIR="$(pwd)"

  dir_appdir_create

  # Download tools
  if [[ "$GIMG_PKG_TYPE" = "appimage" ]]; then
    _fetch_appimagetool
  fi
  rpcs3_download
  _fetch_imagemagick

  # Populate appdir
  files_copy "$name" "$dir" "$bios" "$core" "$cover" "null"

  if [[ "$GIMG_PKG_TYPE" = "flatimage" ]]; then
    runner_create_flatimage "$bios"
    build_flatimage "$name"
  else
    runner_create_appimage "$bios"
    desktop_entry_create "$name"
    build_appimage
  fi
}

main "$@"
