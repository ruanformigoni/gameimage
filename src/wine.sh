#!/usr/bin/env bash

######################################################################
# @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
# @file        : wine
# @created     : Thursday Oct 13, 2022 10:40:26 -03
######################################################################

#shellcheck disable=2155
#shellcheck source=/dev/null

set -e

GIMG_SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

source "$GIMG_SCRIPT_DIR/common.sh"

# Get wine appimage and define wine alias
function wine_download()
{
  local url

  url=$(curl -H "Accept: application/vnd.github+json" \
    https://api.github.com/repos/ruanformigoni/wine/releases 2>&1 |
    grep -Eo "https://.*continuous-.*/wine-$GIMG_WINE_DIST.*\.AppImage\"")

  if [ ! -f "AppDir/usr/bin/wine" ]; then
    _fetch "wine" "${url%\"}"
    mv wine AppDir/usr/bin/wine
    ln -s wine AppDir/usr/bin/winetricks
  fi

  # shellcheck disable=2139
  WINE="$(pwd)/AppDir/usr/bin/wine"
  WINETRICKS="$(pwd)/AppDir/usr/bin/winetricks"
}

function arch_select()
{
  if [ "$GIMG_YAML" ]; then
    yq -e '.arch' "$GIMG_YAML"
  else
    msg "Please select the architecture" >&2
    select i in "win32" "win64"; do
      [ "$i" ] || continue;
      echo "$i"; break
    done
  fi
}

function wine_configure()
{
  export WINEPREFIX="$(pwd)/AppDir/app/wine"

  if [ ! -d "$WINEPREFIX" ]; then
    export WINEARCH="$(arch_select)"

  if [ "$(_select_yn "Download wineprefix?" "y")" != "n" ]; then
      _fetch "prefix.tar.xz" \
        "https://github.com/ruanformigoni/wine/releases/download/continuous-ge/wineprefix-${WINEARCH#win}.tar.xz"
      tar -xf prefix.tar.xz
      mv wine "$(pwd)/AppDir/app/"
      rm prefix.tar.xz
    else
      if [ ! -d "$WINEPREFIX" ]; then
        "$WINETRICKS" fontsmooth=rgb
        "$WINETRICKS" dxvk
      fi
    fi
  fi

  "$WINE" --version

  declare -A opts

  if ! [ "$GIMG_YAML" ]; then
    for i in $("$WINETRICKS" list-all | awk '!/=+/ { print $1 }'); do
      opts["$i"]=1
    done

    pwd
    msg "winetricks, use it to install dependencies, leave it blank to continue"
    while :; do
      echo -n "winetricks> "
      read -r args
      # Stop on empty input
      [ "$args" ] || break
      # Check if is bash cmd
      for i in "${args[@]}"; do
        [ "${opts[$i]}" ] || { eval "${args[*]}" || true; continue 2; }
      done
      # If not call winetricks
      "$WINETRICKS" "$args" || continue
    done
  fi
}

function wine_install()
{
  msg "Showing executable files in $1/rom"
  msg "Select the file to install"
  while :; do
    if [ -n "$GIMG_DIR_ROM_EXTRA" ]; then
      _eval_select 'find -L ' "\"$1/rom\" " "\"$GIMG_DIR_ROM_EXTRA\"" ' -iname "*.exe"' || break
    else
      _eval_select 'find -L ' "\"$1/rom\" " ' -iname "*.exe"' || break
    fi

    [ ! -f "${_FN_RET[0]}" ] && die "No valid file found in $1/rom"

    #shellcheck disable=2005
    echo "$(cd "$(dirname "${_FN_RET[0]}")" && "$WINE" "${_FN_RET[0]}")"
    [ "$(_select_yn "Install another file?" "N")" = "y" ] || break
  done
}

function wine_test()
{
  [ "$(_select_yn "Test the installed software?" "N")" = "y" ] || return 0

  while :; do
    _eval_select "find " "\"$1\"" " -not -path *drive_c/windows/*.exe -iname *.exe" || break
    #shellcheck disable=2005
    echo "$(cd "$(dirname "$_FN_RET")" && "$WINE" "$_FN_RET")"
    [ "$(_select_yn "Test the another file?" "N")" = "y" ] || break
  done
}

function wine_executable_select()
{
  msg "Select the main executable"
  _eval_select 'find "AppDir/app/wine" -not -path "*drive_c/windows/*.exe" -iname "*.exe"'
  executable="$_FN_RET"

  # Get directory to move out from drive c:
  local dir_installation
  dir_installation="$(dirname "$executable")"
  dir_installation="${dir_installation//*drive_c\/}"
  dir_installation="${dir_installation//\/*}"

  # Create directory to store installed files
  dir_target="AppDir/app/rom"
  mkdir -p "$dir_target"

  # Move to target directory
  msg "Moving '$dir_installation' to '$dir_target'"
  mv "AppDir/app/wine/drive_c/$dir_installation" "$dir_target"

  _FN_RET[0]="$dir_installation"
  _FN_RET[1]="$executable"
}

function runner_create()
{
  local path_exec

  # Binary directory path under c: drive
  path_install="$1"

  # Binary path under AppDir
  path_exec="$2"
  path_exec="${path_exec##*AppDir/app/}"

  # Create runner script
  { sed -E 's/^\s+://' | tee AppDir/AppRun; } <<-END
    :#!/usr/bin/env bash
    :
    :set -e
    :
    :# Wine executable
    :WINE="\$APPDIR/usr/bin/wine"
    :
    :if [[ "\$(basename "\${APPIMAGE}")" =~ \.\.AppImage ]]; then
    :  # Set hidden config dir
    :  CFGDIR="\$(dirname "\$APPIMAGE")/\$(basename "\$APPIMAGE").config"
    :else
    :  # Set visible config dir
    :  CFGDIR="\$(dirname "\$APPIMAGE")/.\$(basename "\$APPIMAGE").config"
    :fi
    :
    :# Path to appimage mountpoint
    :MNTDIR="\$APPDIR"
    :
    :# Create wine prefix if not exists
    :export WINEPREFIX="\$CFGDIR/wine"
    :if [ ! -d "\$WINEPREFIX" ]; then
    :  mkdir -p "\$CFGDIR"
    :  cp -r "\$MNTDIR/app/wine" "\$CFGDIR"
    :fi
    :
    :# Create/Update symlink to the application directory
    :rm -f "\$CFGDIR/wine/drive_c/$path_install"
    :ln -s "\$MNTDIR/app/rom/$path_install" "\$CFGDIR/wine/drive_c/$path_install"
    :
    :cd "\$(dirname "\$CFGDIR/$path_exec")"
    :
    :exec="$(basename "$path_exec")"
    :
    :if [ "\$@" ]; then
    :  "\$WINE" "\$@"
    :else
    :  "\$WINE" "\$exec"
    :fi
	END

  # Allow execute
  chmod +x AppDir/AppRun

  msg -n "AppRun written, make further changes to it if you desire, then press enter..."
  read -r
}

function main()
{
  # Validate params
  params_validate "wine" "$@"

  local name="${_FN_RET[0]}"
  local dir="${_FN_RET[1]}"
  local cover="${_FN_RET[4]}"

  # Create dirs
  cd "$(dir_build_create "$dir")"

  dir_appdir_create

  # Download tools
  _fetch_appimagetool
  _fetch_imagemagick

  # Install and configure application
  wine_download
  wine_configure
  wine_install "$dir"
  wine_test "$dir/build/AppDir/app/wine"
  wine_executable_select

  # Create runner script
  runner_create "${_FN_RET[0]}" "${_FN_RET[1]}"

  # Copy cover
  ./imagemagick "$cover" "AppDir/${name}.png"

  # Create desktop entry
  desktop_entry_create "$name"

  # Build appimage
  appdir_build
}

main "$@"

#  vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
