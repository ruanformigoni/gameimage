#!/tmp/gameimage/bin/bash

######################################################################
# @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
# @file        : wine
# @created     : Thursday Oct 13, 2022 10:40:26 -03
######################################################################

#shellcheck disable=2155
#shellcheck source=/dev/null

set -e

shopt -s nullglob

GIMG_SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

source "$GIMG_SCRIPT_DIR/common.sh"

# Default options for GIMG_WINE_DIST
if [[ "$GIMG_PKG_TYPE" = "flatimage" ]]; then
  GIMG_WINE_DIST_VALUES="caffe,vaniglia,soda,ge,staging,tkg,osu-tkg"
else
  GIMG_WINE_DIST_VALUES="caffe,vaniglia,soda,ge,staging"
fi

# Get wine appimage and define wine alias
function _fetch_wine()
{
  if [[ "$GIMG_PKG_TYPE" = "flatimage" ]]; then
    if ! read -r url; then
      die "Could not fetch wine url for '$GIMG_WINE_DIST', valid values are '$GIMG_WINE_DIST_VALUES'"
    fi < <(_fetch_stdout "https://gitlab.com/api/v4/projects/45732205/releases/Continuous" |
        "$GIMG_SCRIPT_DIR"/jq -e -r '.assets.links.[].direct_asset_url | match("'".*wine/continuous/$GIMG_WINE_DIST-.*"'").string')
  else
    if ! read -r url; then
      die "Could not fetch wine url for '$GIMG_WINE_DIST', valid values are '$GIMG_WINE_DIST_VALUES'"
    fi < <(_fetch_stdout "https://api.github.com/repos/ruanformigoni/wine/releases" | 
      "$GIMG_SCRIPT_DIR"/jq -e -r '.[].assets.[].browser_download_url | match(".*wine-'"$GIMG_WINE_DIST"'.*x86_64.AppImage").string | select (.!=null)')
  fi

  if [ ! -f "AppDir/usr/bin/wine" ]; then
    if [ -f "./wine" ]; then
      mv wine AppDir/usr/bin/wine
    else
      if [[ "$GIMG_PKG_TYPE" = "flatimage" ]]; then
        # Fetch packages
        _fetch "wine" "https://gitlab.com/api/v4/projects/45732205/packages/generic/wine/continuous/base-arch.fim"
        _fetch "opt.dwarfs" "$url"
        # Set home directory to build dir
        ./wine fim-config-set home "$DIR_SRC"
        # Merge flatimage with wine binaries
        ./wine fim-include-path ./opt.dwarfs "/opt.dwarfs"
        # Remove downloaded wine binaries
        rm -f ./opt.dwarfs
      else
        _fetch "wine" "$url"
      fi
      mv wine AppDir/usr/bin/wine
    fi
  fi

  # Create winetricks
  local path_winetricks="AppDir/usr/bin/winetricks"
  if [[ "$GIMG_PKG_TYPE" = "flatimage" ]]; then
    { sed -E 's/^\s+://' | tee "$path_winetricks" &>/dev/null; } <<-"END"
    :#!/usr/bin/env bash
    :
    :SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)"
    :
    :PATH="$PATH:/opt/wine/bin" "$SCRIPT_DIR"/wine fim-exec winetricks "$@"
		END
  else
    ln -sf wine AppDir/usr/bin/winetricks
  fi
  chmod +x "$path_winetricks"
}

function arch_select()
{
  if [ "$GIMG_YAML" ]; then
    "$GIMG_SCRIPT_DIR/yq" -e '.arch' "$GIMG_YAML"
  else
    msg "Please select the architecture" >&2
    _select "win32" "win64"
    echo "${_FN_RET[0]}"
  fi
}

function wine_configure()
{
  export WINEPREFIX="$(pwd)/AppDir/app/wine"

  if [ ! -d "$WINEPREFIX" ]; then
    # Update prefix
    export WINEARCH="$(arch_select)"
    # Avoid symlinks
    "$BIN_WINETRICKS" sandbox
    # Leave the root drive binding
    ln -sfT / "$WINEPREFIX/dosdevices/z:"
    # Smooth fonts
    "$BIN_WINETRICKS" fontsmooth=rgb
  fi

  # If the variable is unset, ask
  # If the variable is set, use value to determine if should install or not
  ## DXVK
  if [ -v GIMG_INSTALL_DXVK ]; then
    if [ "$GIMG_INSTALL_DXVK" -eq 1 ]; then
      "$BIN_WINETRICKS" dxvk
    fi
  elif _select_bool "Install dxvk for directx 9/10/11?" "N"; then
    "$BIN_WINETRICKS" dxvk
  fi
  ## VKD3D
  if [ -v GIMG_INSTALL_VKD3D ]; then
    if [ "$GIMG_INSTALL_VKD3D" -eq 1 ]; then
      "$BIN_WINETRICKS" vkd3d
    fi
  elif _select_bool "Install vkd3d for directx 12?" "N"; then
    "$BIN_WINETRICKS" vkd3d
  fi

  # Output current wine version
  "$BIN_WINE" --version

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
    [ "${args[0]}" = "continue"   ] && break
    # Check if is a local command
    [ "${args[0]}" = "wine"       ] && { "$BIN_WINE" "${args[*]:1}" || true; continue; }
    [ "${args[0]}" = "winetricks" ] && { "$BIN_WINETRICKS" "${args[*]:1}" || true; continue; }
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
    if _select_bool "Install $(basename "${_FN_RET[0]}")?" "Y"; then
      echo "$(cd "$(dirname "${_FN_RET[0]}")" && "$BIN_WINE" "${_FN_RET[0]}")"
    fi

    _select_bool "Install another file?" "N" || break
  done
}

function wine_test()
{
  while :; do
    _eval_select "find " "\"$1\"" " -not -path '*drive_c/windows/*.exe' -iname '*.exe'" || break
    #shellcheck disable=2005
    echo "$(cd "$(dirname "$_FN_RET")" && "$BIN_WINE" "$_FN_RET")"
    _select_bool "Test the another file?" "N" || break
  done
}

function wine_executable_select()
{
  msg "Select the main executable"
  _eval_select 'find "AppDir/app/wine" -not -path "*drive_c/windows/*.exe" -iname "*.exe"'
  echo "$_FN_RET"
}

function wine_package_method()
{
  msg "Install method: $GIMG_PKG_METHOD"
  local executable="$1"
  local dir_build="$2"
  local name="$3"

  # Get directory to move out from drive c:
  local dir_installation
  dir_installation="$(dirname "$executable")"
  dir_installation="${dir_installation//*drive_c\/}"
  dir_installation="${dir_installation//\/*}"

  local dir_target="AppDir/app/rom"

  # Create directory to store installed files
  # Move to external prefix or keep it inside appimage
  ## Make it writteable with overlayfs
  if [ "${GIMG_PKG_METHOD}" = "overlayfs" ]; then
    cp "${GIMG_SCRIPT_DIR}/overlayfs" "AppDir/usr/bin"
  ## Make it writteable with unionfs
  elif [ "${GIMG_PKG_METHOD}" = "unionfs" ]; then
    cp "${GIMG_SCRIPT_DIR}/unionfs" "AppDir/usr/bin"
  ## Copy on prefix not found
  elif [ "${GIMG_PKG_METHOD}" = "copy" ]; then
    mkdir -p "$dir_target"
    # Move installed software to target directory
    msg "Moving '$dir_installation' to '$dir_target'"
    mv "AppDir/app/wine/drive_c/$dir_installation" "$dir_target"
  ## Keep prefix as is
  elif [ "${GIMG_PKG_METHOD}" = "prefix" ]; then
    dir_target="$dir_build/.${name}.${GIMG_PKG_TYPE}.config"
    mkdir -p "$dir_target"
    # Move prefix
    msg "Moving '$dir_build/AppDir/app/wine' to '$dir_target'"
    mv "$dir_build/AppDir/app/wine" "$dir_target"/wine
  ## Writteable inside the image (flatimage only)
  elif [ "${GIMG_PKG_METHOD}" = "dynamic" ]; then
    :
  else
    die "Unsupported package method $GIMG_PKG_METHOD"
  fi

  _FN_RET[0]="$dir_installation"
  _FN_RET[1]="$executable"
}

function runner_create()
{
  # Build Dir
  local dir_build="$1"; shift

  # Application name, space separated, uppercase
  local name="$1"; shift
  declare -a name_upper
  for i in $name; do name_upper+=("${i^}"); done
  name="${name_upper[*]}"

  # Binary directory path under c: drive
  path_install="$1"

  # Binary path under AppDir
  local path_exec
  path_exec="$2"
  path_exec="${path_exec##*AppDir/app/}"

  # Parse yaml
  cp "${GIMG_SCRIPT_DIR}/yq" "AppDir/usr/bin"

  # Launcher
  cp "${GIMG_SCRIPT_DIR}/launcher" "AppDir/usr/bin"

  if ! _select_bool "Include wine inside the appimage?" "N"; then
    # Move from default location to build dir
    if [ -f "$BIN_WINE" ]; then mv "$BIN_WINE" "$dir_build"; fi
  fi

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
    :# Exports
    :export DIR_CALL="\$(dirname "\$APPIMAGE")"
    :export DIR_APP="\$APPDIR"
    :if [ -f "\$APPDIR/usr/bin/wine" ]; then
    :  export BIN_WINE="\$APPDIR/usr/bin/wine"
    :fi
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

  if [ "${GIMG_PKG_METHOD}" = "overlayfs" ]; then
    { sed -E 's/^\s+://' | tee -a AppDir/AppRun | sed -e 's/^/-- /'; } <<-END
    :# Unmount after appimage unmounts squashfs
    :function _exit() { pkill -f "overlayfs.*\$WINEPREFIX"; }
    :trap _exit SIGINT EXIT
    :
    :# Configure dirs for overlayfs
    :export WINEPREFIX_RO="\$MNTDIR/app/wine"
    :export WINEPREFIX_RW="\$CFGDIR/union"
    :mkdir -p "\$WINEPREFIX"
    :mkdir -p "\$WINEPREFIX_RW"
    :
    :# Mount prefix with overlayfs
    :# uidmapping and gidmapping: These options specify user and group ID mappings, respectively.
    :# They are used to map UIDs and GIDs from the host to the overlay filesystem. The format is
    :# containerID:hostID:size.
    :# 0:10:100: This maps the first 100 UIDs in the container (0-99) to UIDs 10-109 on the host.
    :# 100:10000:2000: This maps UIDs 100-2099 in the container to UIDs 10000-11999 on the host.
    :"\$APPDIR/usr/bin/overlayfs" -o uidmapping="\$(id -u):\$(id -u):\$(id -u)" \\
    :  -o gidmapping="\$(id -g):\$(id -g):\$(id -g)" \\
    :  -o squash_to_uid="\$(id -u)" \\
    :  -o squash_to_gid="\$(id -g)" \\
    :  -o lowerdir="\$WINEPREFIX_RO",upperdir="\$WINEPREFIX_RW",workdir="\$CFGDIR" "\$WINEPREFIX"
    :
		END
  elif [ "${GIMG_PKG_METHOD}" = "unionfs" ]; then
    { sed -E 's/^\s+://' | tee -a AppDir/AppRun | sed -e 's/^/-- /'; } <<-END
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
  elif [ "${GIMG_PKG_METHOD}" = "copy" ]; then
    { sed -E 's/^\s+://' | tee -a AppDir/AppRun | sed -e 's/^/-- /'; } <<-END
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
    { sed -E 's/^\s+://' | tee -a AppDir/AppRun | sed -e 's/^/-- /'; } <<-END
    :# Requires pre-existing prefix to start
    :[ ! -d "\$WINEPREFIX" ] && { echo "Requires pre-existing prefix to start"; exit 1; }
    :
		END
  fi

  { sed -E 's/^\s+://' | tee -a AppDir/AppRun | sed -e 's/^/-- /'; } <<-END
    :# Enter the main executable's directory
    :cd "\$(dirname "\$CFGDIR/$path_exec")"
    :
    :# Name of the main executable (without path)
    :export GIMG_DEFAULT_EXEC="\${GIMG_DEFAULT_EXEC:-$(basename "$path_exec")}"
    :
    :YQ="\$APPDIR/usr/bin/yq"
    :
    :# Check YAML integrity
    :YAML="\$CFGDIR/config.yml"
    :if ! "\$YQ" --exit-status 'tag == "!!map" or tag == "!!seq"' "\$YAML" &>/dev/null; then
    :  echo "cmd: \"{wine} {exec}\"" > "\$YAML"
    :  echo "runner: ''" >> "\$YAML"
    :  echo "runner_custom: false" >> "\$YAML"
    :fi
    :
    :# Change startup command
    :if [[ "\$1" =~ --gameimage-cmd=(.*) ]]; then
    :  # Define custom command
    :  CMD="\${BASH_REMATCH[1]}"
    :  CMD="\${CMD//\"/\\\\\"}" # Escape quotes
    :  "\$YQ" -i ".cmd = \"\$CMD\"" "\$YAML"
    :  exit 0
    :fi
    :
    :# Change runner
    :if [[ "\$1" =~ --gameimage-runner=(.*) ]]; then
    :  # Define custom runner
    :  RUNNER="\${BASH_REMATCH[1]}"
    :  RUNNER="\${RUNNER//\"/\\\\\"}" # Escape quotes
    :  "\$YQ" -i ".runner = \"\$RUNNER\"" "\$YAML"
    :  exit 0
    :fi
    :
    :# Start application
    :if [ -z "\$GIMG_LAUNCHER_DISABLE" ]; then
    :  LAUNCHER="\$APPDIR/usr/bin/launcher"
    :  export GIMG_CONFIG_FILE="\$CFGDIR/config.yml"
    :  export GIMG_LAUNCHER_NAME="$name"
    :  export GIMG_LAUNCHER_IMG="\$APPDIR/.DirIcon"
    :  export GIMG_LAUNCHER_EXECUTABLES="\$(find . -iname '*.exe' -exec echo -n '{}|' \\;)"
    :  "\$LAUNCHER"
    :else
    :  if [ -z "\$BIN_WINE" ]; then
    :   echo "-- Wine runner is missing"
    :   echo "-- You can set it with './\$(basename "\$APPIMAGE") --gameimage-runner=/path/to/wine'"
    :   echo "-- The path must be absolute"
    :   exit 1
    :  fi
    :
    :  # Parse runner location
    :  if read -r RUNNER < <("\$YQ" -re '.runner | select(.!=null)' "\$YAML" | xargs); then
    :    BIN_WINE="\${RUNNER:-"\$BIN_WINE"}"
    :  fi
    :
    :  # Parse startup command
    :  if read -r CMD < <("\$YQ" -re '.cmd | select(.!=null)' "\$YAML" | xargs); then
    :    # Run custom command, replaces {wine} and {exec} strings
    :    CMD="\${CMD//\{wine\}/\"\$BIN_WINE\"}"
    :    CMD="\${CMD//\{exec\}/\"\\\$GIMG_DEFAULT_EXEC\"}"
    :    CMD="\${CMD//\{here\}/\"\$DIR_CALL\"}"
    :    CMD="\${CMD//\{appd\}/\"\$DIR_APP\"}"
    :  else
    :    echo "Startup command is empty, try to erase YAML"
    :    exit 1
    :  fi
    :
    :  eval "\$CMD"
    :fi
	END

  # Allow execute
  chmod +x AppDir/AppRun

  # Erase symlinks from wine user home
  for i in "$WINEPREFIX/drive_c/users/$(whoami)/"{AppData,Application\ Data,Contacts,Desktop,Documents,Downloads,Favorites,Links,Music,My\ Documents,Pictures,Saved\ Games,Searches,Videos}; do
    # Erase symbolic links
    [ ! -L "$i" ] || rm "$i";
  done

  msg "%b" "AppRun written, make further changes to it if you desire, then press enter..."
  read -r
}

function runner_create_flatimage()
{
  # Build Dir
  local dir_build="$1"; shift

  # Application name, space separated, uppercase
  local name="$1"; shift
  declare -a name_upper
  for i in $name; do name_upper+=("${i^}"); done
  name="${name_upper[*]}"

  # Binary directory path under c: drive
  path_install="$1"

  # Binary path under AppDir
  local path_exec
  path_exec="$2"
  path_exec="${path_exec##*AppDir/app/wine/}"

  # Parse yaml
  "$BIN_PKG" fim-root cp "${GIMG_SCRIPT_DIR}/yq" "\$FIM_DIR_STATIC"

  # Launcher
  "$BIN_PKG" fim-root cp "${GIMG_SCRIPT_DIR}/launcher" "\$FIM_DIR_STATIC/launcher"

  # Create runner script
  { sed -E 's/^\s+://' | tee AppDir/gameimage.sh | sed -e 's/^/-- /'; } <<-END
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
   :# Include wine in PATH
   :PATH="/opt/wine/bin:\$PATH"
   :
   :export LC_ALL="\${LC_ALL:-C}"
   :
   :# Exports
   :export DIR_CALL="\$FIM_DIR_BINARY"
   :export BIN_WINE="/fim/scripts/wine.sh"
   :
   :# Define configuration directory path
   :export CFGDIR="\$FIM_DIR_BINARY/.\$FIM_FILE_BINARY.config"
   :mkdir -p "\$CFGDIR"
   :
	END

  if [[ "$GIMG_PKG_METHOD" = "prefix" ]]; then
    { sed -E 's/^\s+://' | tee -a AppDir/gameimage.sh | sed -e 's/^/-- /'; } <<-END
     :# Export prefix
     :export WINEPREFIX; WINEPREFIX="\$CFGDIR/wine"
     :
     :# Requires pre-existing prefix to start
     :[ ! -d "\$WINEPREFIX" ] && { echo "Requires pre-existing prefix to start"; exit 1; }
     :
		END
  elif [ "$GIMG_PKG_METHOD" = "copy" ]; then
    { sed -E 's/^\s+://' | tee -a AppDir/gameimage.sh | sed -e 's/^/-- /'; } <<-END
    :# Export prefix
    :export WINEPREFIX; WINEPREFIX="\$CFGDIR/wine"
    :
    :# Copy prefix to outside of appimage
    :if [ ! -d "\$WINEPREFIX" ]; then
    :  mkdir -p "\$CFGDIR"/wine
    :  cp -r "/prefix"/. "\$CFGDIR"/wine
    :fi
    :
    :# Create/Update symlink to the application directory
    :ln -T -sfn "/rom/$path_install" "\$CFGDIR/wine/drive_c/$path_install"
    :
		END
  else # overlayfs, dynamic
    { sed -E 's/^\s+://' | tee -a AppDir/gameimage.sh | sed -e 's/^/-- /'; } <<-END
     :# Export prefix
     :export WINEPREFIX; WINEPREFIX="\$(readlink -f "/prefix")"
     :
		END
  fi


  { sed -E 's/^\s+://' | tee -a AppDir/gameimage.sh | sed -e 's/^/-- /'; } <<-END
   :# Enter the main executable's directory
   :cd "\$(dirname "\$WINEPREFIX/$path_exec")"
   :
   :# Name of the main executable (without path)
   :export GIMG_DEFAULT_EXEC="\${GIMG_DEFAULT_EXEC:-$(basename "$path_exec")}"
   :
   :# Check YAML integrity
   :YAML="\$CFGDIR/config.yml"
   :if ! yq --exit-status 'tag == "!!map" or tag == "!!seq"' "\$YAML" &>/dev/null; then
   :  echo "cmd: \"{wine} {exec}\"" > "\$YAML"
   :  echo "runner: ''" >> "\$YAML"
   :  echo "runner_custom: \"false\"" >> "\$YAML"
   :fi
   :
   :# Change startup command
   :if [[ "\$1" =~ --gameimage-cmd=(.*) ]]; then
   :  # Define custom command
   :  CMD="\${BASH_REMATCH[1]}"
   :  CMD="\${CMD//\"/\\\\\"}" # Escape quotes
   :  yq -i ".cmd = \"\$CMD\"" "\$YAML"
   :  exit 0
   :fi
   :
   :# Change runner
   :if [[ "\$1" =~ --gameimage-runner=(.*) ]]; then
   :  # Define custom runner
   :  RUNNER="\${BASH_REMATCH[1]}"
   :  RUNNER="\${RUNNER//\"/\\\\\"}" # Escape quotes
   :  yq -i ".runner = \"\$RUNNER\"" "\$YAML"
   :  exit 0
   :fi
   :
   :# Start application
   :if [ -z "\$GIMG_LAUNCHER_DISABLE" ]; then
   :  export GIMG_CONFIG_FILE="\$CFGDIR/config.yml"
   :  export GIMG_LAUNCHER_NAME="$name"
   :  export GIMG_LAUNCHER_IMG="\$FIM_DIR_MOUNT/fim/desktop/icon.png"
   :  export GIMG_LAUNCHER_EXECUTABLES="\$(find . -iname '*.exe' -exec echo -n '{}|' \;)"
   :  launcher
   :else
   :  if [ -z "\$BIN_WINE" ]; then
   :   echo "-- Wine runner is missing"
   :   echo "-- You can set it with './\$(basename "$FIM_FILE_BINARY") --gameimage-runner=/path/to/wine'"
   :   echo "-- The path must be absolute"
   :   exit 1
   :  fi
   :
   :  # Parse runner location
   :  if read -r RUNNER < <(yq -re '.runner | select(.!=null)' "\$YAML" | xargs); then
   :    BIN_WINE="\${RUNNER:-"\$BIN_WINE"}"
   :  fi
   :  
   :  # Parse startup command
   :  if read -r CMD < <(yq -re '.cmd | select(.!=null)' "\$YAML" | xargs); then
   :    # Run custom command, replaces {wine} and {exec} strings
   :    CMD="\${CMD//\{wine\}/\"\$BIN_WINE\"}"
   :    CMD="\${CMD//\{exec\}/\"\$GIMG_DEFAULT_EXEC\"}"
   :    CMD="\${CMD//\{here\}/\"\$DIR_CALL\"}"
   :  else
   :    echo "Startup command is empty, try to erase YAML"
   :    exit 1
   :  fi
   :
   :  eval "\$CMD"
   :fi
	END

  # Allow execute
  chmod +x AppDir/gameimage.sh

  # Move into flatimage
  "$BIN_PKG" fim-exec mv "$DIR_BUILD/AppDir/gameimage.sh" "/fim/gameimage.sh"

  # Erase symlinks from wine user home
  for i in "$WINEPREFIX/drive_c/users/$(whoami)"/*; do
    if [ -L "$i" ]; then rm "$i"; fi
  done

  # Set default command
  "$BIN_PKG" fim-cmd "/fim/gameimage.sh"

  # Set HOME directory
  #shellcheck disable=2016
  "$BIN_PKG" fim-config-set home '$FIM_DIR_BINARY/.${FIM_FILE_BINARY}.config'

  msg "%b" "Runner script written, make further changes to it if you desire, then press enter..."
  read -r
}

function main()
{
  # Validate params
  params_validate "wine" "$@"

  local name="${_FN_RET[0]}"
  name="${name// /-}"
  local dir_src="${_FN_RET[1]}"
  local cover="${_FN_RET[4]}"

  # Export dir src
  export DIR_SRC="$dir_src"

  # Create & cd in build dir
  export DIR_BUILD="$(dir_build_create "$dir_src")"
  cd "$DIR_BUILD"

  # Revert prefix
  if [[ -d "AppDir/app/rom" ]]; then
    mv -vf "AppDir/app/rom"/* "AppDir/app/wine/drive_c/"
    rmdir "AppDir/app/rom"
  fi

  # Revert copy GIMG_PKG_METHOD
  if [[ -d "$DIR_BUILD/.${name}.${GIMG_PKG_TYPE}.config/wine" ]]; then
    # If it exists, just erase the one from the config dir
    if [[ -d "$DIR_BUILD/AppDir/app/wine" ]]; then
      msg "Remove $DIR_BUILD/.${name}.${GIMG_PKG_TYPE}.config/wine"
      rm -rf "$DIR_BUILD/.${name}.${GIMG_PKG_TYPE}.config/wine"
    else
      msg "Move '$DIR_BUILD/.${name}.${GIMG_PKG_TYPE}.config/wine' to '$DIR_BUILD/AppDir/app'"
      mv -f "$DIR_BUILD/.${name}.${GIMG_PKG_TYPE}.config/wine" "$DIR_BUILD/AppDir/app"
    fi
  fi

  export WINEPREFIX="$(pwd)/AppDir/app/wine"
  export BIN_WINE="$(pwd)/AppDir/usr/bin/wine"
  export BIN_WINETRICKS="$(pwd)/AppDir/usr/bin/winetricks"

  # Check if wine was moved by the build stage
  if [ -f "$DIR_BUILD"/wine ]; then
    mv "$DIR_BUILD"/wine "$BIN_WINE" 
  fi

  dir_appdir_create

  while :; do

    case "$GIMG_STAGE" in
      fetch) 
        # Download tools
        if [[ "$GIMG_PKG_TYPE" = "appimage" ]]; then
          _fetch_appimagetool
        fi
        _fetch_imagemagick
        _fetch_wine
      ;;

      configure)
        # Install and configure application
        wine_configure
      ;;

      install)
        wine_install "$dir_src"
      ;;

      test)
        wine_test "$DIR_BUILD/AppDir/app/wine"
      ;;

      build)
        # Select main executable
        local path_executable="$(wine_executable_select "$DIR_BUILD" "$name")"

        # Adjust tree structure based on package method
        wine_package_method "$path_executable" "$DIR_BUILD" "$name"
        local dir_installation="${_FN_RET[0]}"
        local basename_executable="${_FN_RET[1]}"

        # Convert cover
        ./imagemagick "$cover" -resize '600x900^' -gravity center -extent 600x900 "AppDir/${name}.png"

        # Generated image name
        local name_image="${name}.${GIMG_PKG_TYPE}"

        if [[ "$GIMG_PKG_TYPE" = "flatimage" ]]; then
          # Define path to release package
          export BIN_PKG="$DIR_BUILD/$name_image"
          # Copy wine to build dir
          cp "$BIN_WINE" "$BIN_PKG"
          # Copy cover
          "$BIN_PKG" fim-exec mkdir -p /fim/desktop
          "$BIN_PKG" fim-exec cp "AppDir/${name}.png" /fim/desktop/icon.png
          # Compress & include prefix 
          build_flatimage_wine
          # Create runner script
          runner_create_flatimage "$DIR_BUILD" "$name" "$dir_installation" "$basename_executable"
          # Set application info
          "$BIN_PKG" fim-config-set name "$name"
          # shellcheck disable=2016
          "$BIN_PKG" fim-config-set icon '"$FIM_DIR_MOUNT"/fim/desktop/icon.png'
          "$BIN_PKG" fim-config-set categories "Game"
          "$BIN_PKG" fim-config-set desktop 1
          # Print finished status
          msg "Created '$BIN_PKG'!"
        elif [[ "$GIMG_PKG_TYPE" = "appimage" ]]; then
          # Create runner script
          runner_create "$DIR_BUILD" "$name" "$dir_installation" "$basename_executable"
          # Create desktop entry
          desktop_entry_create "$name"
          # Build appimage
          build_appimage
          # Remove if exists
          rm -f "$name_image"
          # Rename created image
          mv ./*.AppImage "$name_image"
          # Print finished status
          msg "Created '$name_image'!"
        else
          die "Invalid package type '$GIMG_PKG_TYPE'"
        fi
      ;;
    esac

    # Allow for single-stage execution without user input (for the GUI)
    if [ -n "$GIMG_STAGE_SINGLE" ]; then
      break
    else
      msg "Select one of the stages listed below"
      _select "fetch" "configure" "install" "test" "build"
      GIMG_STAGE="${_FN_RET[0]}"
    fi
  done
}

main "$@"

#  vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :
