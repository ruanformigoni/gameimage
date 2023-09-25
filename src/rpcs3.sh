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

  url="$("$GIMG_SCRIPT_DIR"/busybox wget --header="Accept: application/vnd.github+json" -O - \
    https://api.github.com/repos/RPCS3/rpcs3-binaries-linux/releases/latest 2>&1 |
    grep -o "https://.*\.AppImage")"

  msg "rpcs3: ${url}"

  # Get rpcs3
  if [ ! -f "AppDir/usr/bin/rpcs3" ]; then
    if [ ! -f "rpcs3.AppImage" ]; then
      # Get AppImage of rpcs3
      "$GIMG_SCRIPT_DIR"/busybox wget -O rpcs3.AppImage "$url"

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

  dir_appdir_create

  # Download tools
  _fetch_appimagetool
  rpcs3_download
  _fetch_imagemagick

  # Populate appdir
  files_copy "$name" "$dir" "$bios" "$core" "$cover" "null"

  runner_create "$bios"

  desktop_entry_create "$name"

  # Build appimage
  appdir_build
}

main "$@"
