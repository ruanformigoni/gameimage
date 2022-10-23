#!/usr/bin/env bash

######################################################################
# @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
# @file        : common
# @created     : Tuesday Oct 04, 2022 02:49:02 -03
######################################################################

# shellcheck disable=2155

set -e

PID="$$"

function die()
{
  kill -s SIGTERM "$PID"
  exit 1
}

function msg()
{
  # Test for color support
  if [ "$(tput colors)" -ge 8 ]; then
    echo -e "[\033[32m*\033[m] $*" >&2
  else
    echo "[*] $*" >&2
  fi
}

function is_digit()
{
  [[ "${*}" =~ [0-9]+ ]] && echo true || echo false
}

function params_validate()
{
  local platform="$1"; shift

  # Convert path to absolute
  local src_dir="$(readlink -f "$2")"

  [ -d "$src_dir" ] || { msg "Invalid src dir ${src_dir}"; die; }

  local rom
  if [ "$platform" = "wine" ]; then
    rom="null"
  elif [ ! -d "$src_dir/rom" ]; then
    msg "Directory \"$src_dir/rom\" not found"; die; 
  else
    declare -a files

    readarray -t files < <(find "$src_dir/rom" -maxdepth 1 -type f)
    [[ "${#files[@]}" -ne 0 ]] || { msg "No file found in rom directory $src_dir/rom"; die; }

    if [[ "${#files[@]}" -eq 1 ]]; then
      rom="${files[0]}"
    else
      msg "Select the rom file to boot when the appimage is clicked"
      msg "It must be a number between 1 and ${#files[@]}"
      msg "Tip: In retroarch, you can change discs with F1 -> disc control -> load new disc"

      select i in "${files[@]}"; do
        [[ "$(is_digit "$i")" = "true" ]] || continue
        rom="$i"
        break
      done
    fi

    msg "Selected rom: $rom"
    [ -f "$rom" ] || { msg "Invalid rom file: $rom"; die; }
  fi

  local core
  if [ -d "$src_dir/core" ]; then
    read -r core <<< "$(find "$src_dir/core" -regextype posix-extended -iregex ".*so" -print -quit)"
    [ -f "$core" ] || { msg "Invalid core file: $core"; die; }
    msg "Selected core: $core"
  else
    core="null"
  fi

  local cover
  if [ ! -d "$src_dir/icon" ]; then
    msg "Directory \"$src_dir/icon\" not found"; die; 
  else
    read -r cover <<< "$(find "$src_dir/icon" -regextype posix-extended -iregex ".*(jpg|png|svg)" -print -quit)"
    [ -f "$cover" ] || { msg "Invalid cover file: $cover"; die; }
    msg "Selected cover: $cover"
  fi

  local bios
  if [ -d "$src_dir/bios" ]; then
    read -r bios <<< "$(find "$src_dir/bios" -regextype posix-extended -iregex ".*(bin|pup)" -print -quit)"
    [ -f "$bios" ] || { msg "Invalid bios file: $bios"; die; }
    msg "Selected bios: $bios"
  else
    bios="null"
  fi

  # Get name and normalize to dash separated lowercase
  local name="${1// /-}"
  local name="$(echo "$name" | tr '[:upper:]' '[:lower:]')"

  # Return
  echo -e "$name\n$src_dir\n$bios\n$core\n$cover\n$rom"
}

function dir_build_create()
{
  local build_dir="build"

  mkdir -p "$build_dir"

  msg "build dir: $(readlink -f ./"${build_dir}")"

  echo "$build_dir"
}

function dir_appdir_create()
{
  local appdir="AppDir"

  if [ -d "$appdir" ]; then
    echo -n "AppDir from previous run found, remove it? [y/N]: "
    read -r opt
    [ "$opt" = "y" ] && rm -rf "$appdir";
  fi

  mkdir -p AppDir
  mkdir -p AppDir/app
  mkdir -p AppDir/usr/bin
  mkdir -p AppDir/usr/share/icons
}

function appimagetool_download()
{
  # Get appimagetool
  [ ! -f "./appimagetool" ] && wget -q --show-progress --progress=bar:noscroll -O appimagetool https://github.com/AppImage/AppImageKit/releases/download/continuous/appimagetool-x86_64.AppImage

  # Make executable
  chmod +x appimagetool
}

function files_copy()
{
  local name="$1"
  local dir="$2"
  local bios="$3"
  local core="$4"
  local cover="$5"

  # Rom
  cp -r "$dir"/rom/* AppDir/app/

  # Copy image to AppDir
  ## Get imagemagick
  wget -q --show-progress --progress=bar:noscroll -O imagemagick https://imagemagick.org/archive/binaries/magick
  chmod +x imagemagick
  ## Convert image to png
  ./imagemagick "$cover" AppDir/"${name}".png

  # Bios
  if [ "$bios" != "null" ]; then
    cp "$bios" AppDir/app/
  fi

  # Core
  if [ "$core" != "null" ]; then
    cp "$core" AppDir/app/
  fi
}

function desktop_entry_create()
{
  local name

  # Make alt name capital, space separated
  name="$1"
  name="${name//-/ }"
  declare -a name_alt
  for i in $name; do name_alt+=("${i^}"); done
  name="$1"

  # Create runner script
  { sed -E 's/^\s+://' | tee AppDir/"${name}.desktop"; } <<-END
    :[Desktop Entry]
    :Name=${name_alt[*]}
    :Exec=/usr/bin/bash
    :Icon=${name}
    :Type=Application
    :Categories=Utility;
	END
}

function appdir_build()
{
  ARCH=x86_64 ./appimagetool AppDir
}
