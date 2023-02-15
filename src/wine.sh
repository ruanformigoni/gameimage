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
    grep -Eo "https://.*continuous-.*/wine-$GIMG_WINE_DIST-[0-9.-]+-continuous-x86_64.AppImage\"" ||
    die "Error fetching wine url")

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
    "$GIMG_SCRIPT_DIR/yq" -e '.arch' "$GIMG_YAML"
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

  # Output current wine version
  "$WINE" --version

  # Disable wine config update
  echo "disable" > "$WINEPREFIX/.update-timestamp" 

  local dir_current="$(pwd)"
  msg "configuration phase, use it to install dependencies, type continue to skip"
  msg "Commands are 'winetricks ...' and 'wine ...', example 'wine winecfg'"
  msg "You can also type any bash cmd, e.g., 'ls -l'"
  while :; do
    # Read user input
    echo -n "config> "
    read -ra args
    # Newline for gui
    [ -n "$GIMG_GUI" ] && echo "";
    # Stop on continue
    [ "${args[0]}" = "continue" ] && break
    # Check if is a local command
    [ "${args[0]}" = "wine" ] && { "$WINE" "${args[*]:1}" || true; continue; }
    [ "${args[0]}" = "winetricks" ] && { "$WINETRICKS" "${args[*]:1}" || true; continue; }
    # Use it as a bash command
    eval "${args[*]}" || continue
  done
  cd "$dir_current"
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
  msg "Install method: $GIMG_PKG_TYPE"
  msg "Select the main executable"
  _eval_select 'find "AppDir/app/wine" -not -path "*drive_c/windows/*.exe" -iname "*.exe"'
  local executable="$_FN_RET"
  local dir_build="$1"
  local name="$2"

  # Get directory to move out from drive c:
  local dir_installation
  dir_installation="$(dirname "$executable")"
  dir_installation="${dir_installation//*drive_c\/}"
  dir_installation="${dir_installation//\/*}"

  # Create directory to store installed files
  # Move to external prefix or keep it inside appimage
  if [ "${GIMG_PKG_TYPE}" = "unionfs" ]; then
    cp "${GIMG_SCRIPT_DIR}/unionfs" "AppDir/usr/bin"
  elif [ "${GIMG_PKG_TYPE}" = "readonly" ]; then
    dir_target="AppDir/app/rom"
    mkdir -p "$dir_target"
    # Move installed software to target directory
    msg "Moving '$dir_installation' to '$dir_target'"
    mv "AppDir/app/wine/drive_c/$dir_installation" "$dir_target"
  else # prefix
    dir_target="$dir_build/.${name}.AppImage.config"
    mkdir -p "$dir_target"
    # Move prefix to outside of AppImage
    msg "Moving '$dir_build/AppDir/app/wine' to '$dir_target'"
    mv "$dir_build/AppDir/app/wine" "$dir_target"
  fi

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

  # Parse yaml
  cp "${GIMG_SCRIPT_DIR}/yq" "AppDir/usr/bin"

  # Create runner script
  { sed -E 's/^\s+://' | tee AppDir/AppRun; } <<-END
    :#!/usr/bin/env bash
    :
    :set -e
    :
    :# Wine executable
    :DIR_CALL="\$(dirname "\$APPIMAGE")"
    :DIR_APP="\$APPDIR"
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
    :# Remove invalid characters
    :CFGDIR="\${CFGDIR//:/}"
    :
    :# Path to appimage mountpoint
    :MNTDIR="\$APPDIR"
    :
    :# Export prefix
    :export WINEPREFIX="\$CFGDIR/wine"
    :
	END

  if [ "${GIMG_PKG_TYPE}" = "unionfs" ]; then
    { sed -E 's/^\s+://' | tee -a AppDir/AppRun; } <<-END
    :# Unmount after appimage unmounts squashfs
    :function _exit() { pkill -f "unionfs.*\$WINEPREFIX"; }
    :trap _exit SIGINT EXIT
    :
    :# Configure dirs for unionfs
    :export WINEPREFIX_RO="\$MNTDIR/app/wine"
    :export WINEPREFIX_RW="\$CFGDIR/union"
    :mkdir -p "\$WINEPREFIX"
    :mkdir -p "\$WINEPREFIX_RW"
    :
    :# Mount prefix with unionfs
    :"\$APPDIR/usr/bin/unionfs" -o uid="\$(id -u)",gid="\$(id -g)" -ocow "\$WINEPREFIX_RW"=RW:"\$WINEPREFIX_RO"=RO "\$WINEPREFIX"
    :
		END
  elif [ "${GIMG_PKG_TYPE}" = "readonly" ]; then
    { sed -E 's/^\s+://' | tee -a AppDir/AppRun; } <<-END
    :# Copy prefix to outside of appimage
    :if [ ! -d "\$WINEPREFIX" ]; then
    :  mkdir -p "\$CFGDIR"
    :  cp -r "\$MNTDIR/app/wine" "\$CFGDIR"
    :fi
    :
    :# Create/Update symlink to the application directory
    :ln -sf "\$MNTDIR/app/rom/$path_install" "\$CFGDIR/wine/drive_c/$path_install"
    :
		END
  else # prefix
    { sed -E 's/^\s+://' | tee -a AppDir/AppRun; } <<-END
    :# Requires pre-existing prefix to start
    :[ ! -d "\$WINEPREFIX" ] && { echo "Requires pre-existing prefix to start"; exit 1; }
    :
		END
  fi

  { sed -E 's/^\s+://' | tee -a AppDir/AppRun; } <<-END
    :cd "\$(dirname "\$CFGDIR/$path_exec")"
    :
    :EXEC="$(basename "$path_exec")"
    :
    :# Avoid symlink creation
    :mkdir -p "\$WINEPREFIX/drive_c/users/\$(whoami)/"{AppData,Application\ Data,Contacts,Desktop,Documents,Downloads,Favorites,Links,Music,My\ Documents,Pictures,Saved\ Games,Searches,Videos}
    :
    :YQ="\$APPDIR/usr/bin/yq"
    :
    :# Check YAML integrity
    :YAML="\$CFGDIR/config.yml"
    :"\$YQ" --exit-status 'tag == "!!map" or tag == "!!seq"' "\$YAML" &>/dev/null || echo "cmd: \"{wine} {exec}\"" > "\$YAML"
    :
    :if [[ "\$1" =~ --gameimage-cmd=(.*) ]]; then
    :  # Define custom Command
    :  YAML_CMD="\${BASH_REMATCH[1]}"
    :  YAML_CMD="\${YAML_CMD//\"/\\\\\"}" # Escape quotes
    :  "\$YQ" -i ".cmd = \"\$YAML_CMD\"" "\$YAML"
    :elif { YAML_CMD="\$("\$YQ" '.cmd | select(.!=null)' "\$YAML")"; [ -n "\$YAML_CMD" ]; }; then
    :  # Run custom command, replaces {wine} and {exec} strings
    :  YAML_CMD="\${YAML_CMD//\{wine\}/\"\$WINE\"}"
    :  YAML_CMD="\${YAML_CMD//\{exec\}/\"\$EXEC\"}"
    :  YAML_CMD="\${YAML_CMD//\{here\}/\"\$DIR_CALL\"}"
    :  YAML_CMD="\${YAML_CMD//\{appd\}/\"\$DIR_APP\"}"
    :  eval "\$YAML_CMD"
    :else
    :  # Error
    :  echo "Could not start gameimage, remove \$YAML and try again"
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
  local dir_src="${_FN_RET[1]}"
  local cover="${_FN_RET[4]}"

  # Create & cd in build dir
  local dir_build="$(dir_build_create "$dir_src")"
  cd "$dir_build"

  dir_appdir_create

  # Download tools
  _fetch_appimagetool
  _fetch_imagemagick

  # Install and configure application
  wine_download
  wine_configure
  wine_install "$dir_src"
  wine_test "$dir_build/AppDir/app/wine"
  wine_executable_select "$dir_build" "$name"

  # Create runner script
  runner_create "${_FN_RET[0]}" "${_FN_RET[1]}"

  # Copy cover
  ./imagemagick "$cover" "AppDir/${name}.png"

  # Create desktop entry
  desktop_entry_create "$name"

  # Build appimage
  appdir_build

  # Rename AppImage file
  [ -f "${name}.AppImage" ] && rm "${name}.AppImage"
  mv ./*.AppImage "${name}.AppImage"
}

main "$@"

#  vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
