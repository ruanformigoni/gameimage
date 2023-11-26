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

  if [[ "$GIMG_PKG_TYPE" = "flatimage" ]]; then
    url="$("$GIMG_SCRIPT_DIR"/busybox wget -q --header="Accept: application/vnd.github+json" -O - \
      https://api.github.com/repos/flatimage/flatimage-yuzu/releases/latest 2>&1 |
      jq -r ".assets.[0].browser_download_url")"
  else
    url="$("$GIMG_SCRIPT_DIR"/busybox wget --header="Accept: application/vnd.github+json" -O - \
      https://api.github.com/repos/yuzu-emu/yuzu-mainline/releases 2>&1 |
      grep -o "https://.*\.AppImage" | sort | tail -n1)"
  fi

  msg "yuzu: ${url}"

  # Get yuzu
  if [ ! -f "AppDir/usr/bin/yuzu" ]; then
    _fetch "./AppDir/usr/bin/yuzu" "$url"
  fi
}

function runner_create_appimage()
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

  (
    #shellcheck disable=2030
    export XDG_CONFIG_HOME="$(pwd)"/AppDir/app/config
    #shellcheck disable=2030
    export HOME="$(pwd)"/AppDir/app/config/home
    AppDir/usr/bin/yuzu
  )

  local bios="${bios#null}"

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

function runner_create_flatimage()
{
  local bios="$(basename "$1")"
  local keys="$(basename "$2")"
  local rom="$(basename "$3")"

  msg "Install the updates and DLC to pack into the AppImage (not the game itself)"

  mkdir -p AppDir/app/config
  mkdir -p AppDir/app/data/yuzu/nand/system/Contents/registered
  mkdir -p AppDir/app/data/yuzu/keys

  cp AppDir/app/bios/* AppDir/app/data/yuzu/nand/system/Contents/registered
  cp AppDir/app/keys/* AppDir/app/data/yuzu/keys

  (
    #shellcheck disable=2031
    export XDG_CONFIG_HOME="$(pwd)"/AppDir/app/config
    #shellcheck disable=2031
    export XDG_DATA_HOME="$(pwd)"/AppDir/app/data
    AppDir/usr/bin/yuzu
  )

  local bios="${bios#null}"

  # Create runner script
  { sed -E 's/^\s+://' | tee "$BUILD_DIR"/gameimage.sh | sed -e 's/^/-- /'; } <<-END
    :#!/usr/bin/env bash
    :
    :set -e
    :
    :# Path to yuzu
    :export PATH="/yuzu/bin:\$PATH"
    :
    :# Make sure HOME exists
    :mkdir -p "\$HOME"
    :
    :# Set gameimage config dir
    :GIMG_CONFIG_DIR="\${FIM_DIR_BINARY}/.\${FIM_FILE_BINARY}.config"
    :
    :# Set .config
    :export XDG_CONFIG_HOME="\$GIMG_CONFIG_DIR/overlay.app/mount/config"
    :echo "XDG_CONFIG_HOME: \${XDG_CONFIG_HOME}"
    :
    :# Set .local/share
    :export XDG_DATA_HOME="\$GIMG_CONFIG_DIR/overlay.app/mount/data"
    :echo "XDG_DATA_HOME: \${XDG_DATA_HOME}"
    :
    :if [[ "\$*" = "--config" ]]; then
    :  yuzu
    :elif [[ "\$*" ]]; then
    :  yuzu "\$@"
    :else
    :  yuzu -f -g "\$FIM_DIR_MOUNT/app/rom/${rom}"
    :fi
	END

  # Allow execute
  chmod +x "$BUILD_DIR"/gameimage.sh
}

function build_flatimage()
{
  local name="$1"
  local bin_pkg="$BUILD_DIR"/yuzu

  # Copy vanilla yuzu package
  cp "$BUILD_DIR/AppDir/usr/bin/yuzu" "$bin_pkg"

  # Compress game dir
  "$bin_pkg" fim-exec mkdwarfs -f -i "$BUILD_DIR"/AppDir/app -o "$BUILD_DIR"/app.dwarfs

  # Include inside image
  "$bin_pkg" fim-include-path "$BUILD_DIR"/app.dwarfs /app.dwarfs

  # Include launche script
  "$bin_pkg" fim-root mkdir -p /fim/scripts
  "$bin_pkg" fim-root cp "$BUILD_DIR/gameimage.sh" /fim/scripts

  # Default command -> runner script
  "$bin_pkg" fim-cmd /fim/scripts/gameimage.sh

  # Configure overlay
  "$bin_pkg" fim-config-set overlay.app 'App Overlay'
  # shellcheck disable=2016
  "$bin_pkg" fim-config-set overlay.app.host '${FIM_DIR_BINARY}/.${FIM_FILE_BINARY}.config/overlay.app'
  # shellcheck disable=2016
  "$bin_pkg" fim-config-set overlay.app.cont '/app'

  # Copy cover
  "$bin_pkg" fim-exec mkdir -p /fim/desktop-integration
  "$bin_pkg" fim-exec cp "$BUILD_DIR/AppDir/${name}.png" /fim/desktop-integration/icon.png

  # Rename binary
  mv "$bin_pkg" "$BUILD_DIR/${name}.fim"
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

  BUILD_DIR="$(pwd)"

  dir_appdir_create

  # Download tools
  if [[ "$GIMG_PKG_TYPE" = "appimage" ]]; then
    _fetch_appimagetool
  fi
  yuzu_download
  _fetch_imagemagick

  # Populate appdir
  files_copy "$name" "$dir" "$bios" "null" "$cover" "$keys"

  if [[ "$GIMG_PKG_TYPE" = "flatimage" ]]; then
    runner_create_flatimage "$bios" "$keys" "$rom"
    build_flatimage "$name"
  else
    runner_create_appimage "$bios" "$keys" "$rom"
    desktop_entry_create "$name"
    build_appimage
  fi

}

main "$@"
