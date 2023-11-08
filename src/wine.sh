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
function _fetch_wine()
{
  local url

  url=$("$GIMG_SCRIPT_DIR"/busybox wget --header="Accept: application/vnd.github+json" -O - \
    https://api.github.com/repos/ruanformigoni/wine/releases 2>&1 |
    grep -Eo "https://.*continuous-.*/wine-$GIMG_WINE_DIST-[0-9.-]+-continuous-x86_64.AppImage\"" ||
    die "Error fetching wine url")

  if [ ! -f "AppDir/usr/bin/wine" ]; then
    if [ -f "./wine" ]; then
      mv wine AppDir/usr/bin/wine
    else
      _fetch "wine" "${url%\"}"
      mv wine AppDir/usr/bin/wine
    fi
  fi

  if [ ! -f "AppDir/usr/bin/winetricks" ]; then
    ln -sf wine AppDir/usr/bin/winetricks
  fi
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
    "$BIN_WINETRICKS" fontsmooth=rgb
  fi

  # If the variable is unset, ask
  # If the variable is set, use value to determine if should install or not
  ## DXVK
  if [ -v GIMG_INSTALL_DXVK ]; then
    if [ "$GIMG_INSTALL_DXVK" -eq 1 ]; then
      "$BIN_WINETRICKS" -f dxvk
    fi
  elif [ "$(_select_yn "Install dxvk for directx 9/10/11?" "Y")" = "y" ]; then
    "$BIN_WINETRICKS" -f dxvk
  fi
  ## VKD3D
  if [ -v GIMG_INSTALL_VKD3D ]; then
    if [ "$GIMG_INSTALL_VKD3D" -eq 1 ]; then
      "$BIN_WINETRICKS" -f vkd3d
    fi
  elif [ "$(_select_yn "Install vkd3d for directx 12?" "Y")" = "y" ]; then
    "$BIN_WINETRICKS" -f vkd3d
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
    if [ "$(_select_yn "Install $(basename "${_FN_RET[0]}")?" "Y")" = "y" ]; then
      echo "$(cd "$(dirname "${_FN_RET[0]}")" && "$BIN_WINE" "${_FN_RET[0]}")"
    fi

    [ "$(_select_yn "Install another file?" "N")" = "y" ] || break
  done
}

function wine_test()
{
  [ "$(_select_yn "Test the installed software?" "N")" = "y" ] || return 0

  while :; do
    _eval_select "find " "\"$1\"" " -not -path *drive_c/windows/*.exe -iname *.exe" || break
    #shellcheck disable=2005
    echo "$(cd "$(dirname "$_FN_RET")" && "$BIN_WINE" "$_FN_RET")"
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
  if [ "${GIMG_PKG_TYPE}" = "overlayfs" ]; then
    cp "${GIMG_SCRIPT_DIR}/overlayfs" "AppDir/usr/bin"
  elif [ "${GIMG_PKG_TYPE}" = "unionfs" ]; then
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

  include_wine="$(_select_yn "Include wine inside the appimage?" "N")"

  if [ "$include_wine" != "y" ]; then
    # Move from default location to build dir
    if [ -f "$BIN_WINE" ]; then mv "$BIN_WINE" "$dir_build"; fi
  fi

  # Create runner script
  { sed -E 's/^\s+://' | tee AppDir/AppRun | sed -e 's/^/-- /'; } <<-END
    :#!/usr/bin/env bash
    :
    :set -e
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

  if [ "${GIMG_PKG_TYPE}" = "overlayfs" ]; then
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
  elif [ "${GIMG_PKG_TYPE}" = "unionfs" ]; then
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
  elif [ "${GIMG_PKG_TYPE}" = "readonly" ]; then
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
    :  echo "runner_default: \"true\"" >> "\$YAML"
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
    :# Parse runner location
    :if { RUNNER="\$("\$YQ" '.runner | select(.!=null)' "\$YAML")"; [ -n "\$RUNNER" ]; }; then
    :  BIN_WINE="\$RUNNER"
    :fi
    :
    :# Parse startup command
    :if { CMD="\$("\$YQ" '.cmd | select(.!=null)' "\$YAML")"; [ -n "\$CMD" ]; }; then
    :  # Run custom command, replaces {wine} and {exec} strings
    :  CMD="\${CMD//\{wine\}/\"\$BIN_WINE\"}"
    :  CMD="\${CMD//\{exec\}/\"\\\$GIMG_DEFAULT_EXEC\"}"
    :  CMD="\${CMD//\{here\}/\"\$DIR_CALL\"}"
    :  CMD="\${CMD//\{appd\}/\"\$DIR_APP\"}"
    :else
    :  echo "Startup command is empty, try to erase YAML"
    :  exit 1
    :fi
    :
    :# Start application
    :if [ -z "\$GIMG_LAUNCHER_DISABLE" ]; then
    :  LAUNCHER="\$APPDIR/usr/bin/launcher"
    :  export GIMG_CONFIG_FILE="\$CFGDIR/config.yml"
    :  export GIMG_LAUNCHER_NAME="$name"
    :  export GIMG_LAUNCHER_IMG="\$APPDIR/.DirIcon"
    :  export GIMG_LAUNCHER_CMD="\$CMD"
    :  export GIMG_LAUNCHER_EXECUTABLES="\$(find . -iname '*.exe' -exec echo -n '{}|' \\;)"
    :  "\$LAUNCHER"
    :else
    :  if [ -z "\$BIN_WINE" ]; then
    :   echo "-- Wine runner is missing"
    :   echo "-- You can set it with './\$(basename "\$APPIMAGE") --gameimage-runner=/path/to/wine'"
    :   echo "-- The path must be absolute"
    :   exit 1
    :  fi
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

  export WINEPREFIX="$(pwd)/AppDir/app/wine"
  export BIN_WINE="$(pwd)/AppDir/usr/bin/wine"
  export BIN_WINETRICKS="$(pwd)/AppDir/usr/bin/winetricks"

  # Check if wine was moved by the build stage
  if [ -f "$dir_build"/wine ]; then
    mv "$dir_build"/wine "$BIN_WINE" 
  fi

  dir_appdir_create

  while :; do

    case "$GIMG_STAGE" in
      fetch) 
        # Download tools
        _fetch_appimagetool
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
        wine_test "$dir_build/AppDir/app/wine"
      ;;

      build)
        # Select main executable
        wine_executable_select "$dir_build" "$name"

        # Create runner script
        runner_create "$dir_build" "$name" "${_FN_RET[0]}" "${_FN_RET[1]}"

        # Copy cover
        ./imagemagick "$cover" "AppDir/${name}.png"

        # Create desktop entry
        desktop_entry_create "$name"

        # Build appimage
        appdir_build

        # Rename AppImage file
        [ -f "${name}.AppImage" ] && rm "${name}.AppImage"
        mv ./*.AppImage "${name// /-}.AppImage"
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
