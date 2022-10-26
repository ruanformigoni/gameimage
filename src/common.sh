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

function extract()
{
  msg "Extracting $1 file '$2' to $3"

  if [ "$1" = ".zip" ]; then
    unzip -q -o "$2" -d "$3"
  elif [ "$1" = ".7z" ]; then
    7z -aoa x "$2" -o"$3"
  else
    msg "Invalid extract file format"
    die
  fi
}

function param_validate()
{
  local directory="$1"
  local pattern="$2"
  local required="$3"

  if [ -d "$src_dir/$directory" ]; then
    read -r query <<< "$(find "$src_dir/$directory" -regextype posix-extended -iregex "$pattern" -print -quit)"
    [ -f "$query" ] || { msg "Invalid $directory file"; die; }
    msg "Selected $directory: $query"
    echo "$query"
  else
    [ "$required" ] && die || echo "null"
  fi
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
        [ -f "$i" ] || { msg "Invalid selection"; continue; }
        rom="$i"
        break
      done
    fi

    msg "Selected rom: $rom"
  fi

  local core="$(param_validate "core" ".*\.so")"

  local cover="$(param_validate "icon" ".*(\.jpg|\.png|\.svg)" "required")"

  local bios="$(param_validate "bios" ".*(\.bin|\.pup|\.zip|\.7z)")"

  local keys="$(param_validate "keys" ".*(\.zip|\.7z)")"

  # Get name and normalize to dash separated lowercase
  local name="${1// /-}"
  local name="$(echo "$name" | tr '[:upper:]' '[:lower:]')"

  # Return
  echo -e "$name\n$src_dir\n$bios\n$core\n$cover\n$rom\n$keys"
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
  local url

  url="https://github.com/AppImage/AppImageKit/releases/download/continuous/appimagetool-x86_64.AppImage"

  msg "appimagetool: $url"

  # Get appimagetool
  [ ! -f "./appimagetool" ] && wget -q --show-progress --progress=bar:noscroll -O appimagetool "$url"

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
  local keys="$6"

  # Rom
  mkdir -p AppDir/app/rom
  cp -r "$dir"/rom/* AppDir/app/rom

  # Copy image to AppDir
  local url="https://imagemagick.org/archive/binaries/magick"
  msg "imagemagick: ${url}"
  ## Get imagemagick
  if [ ! -f "imagemagick" ]; then
    wget -q --show-progress --progress=bar:noscroll -O imagemagick "$url"
    chmod +x imagemagick
  fi
  ## Convert image to png
  ./imagemagick "$cover" AppDir/"${name}".png

  # Bios
  if [ "$bios" != "null" ]; then
    mkdir -p AppDir/app/bios
    if [[ "$bios" =~ (\.zip|\.7z) ]]; then
      extract "${BASH_REMATCH[1]}" "$bios" AppDir/app/bios
    else
      cp "$bios" AppDir/app/bios
    fi
  fi

  # Keys [yuzu]
  if [ "$keys" != "null" ]; then
    mkdir -p AppDir/app/keys
    if [[ "$keys" =~ (\.zip|\.7z) ]]; then
      extract "${BASH_REMATCH[1]}" "$keys" AppDir/app/keys
    else
      cp "$keys" AppDir/app/keys
    fi
  fi

  # Core
  if [ "$core" != "null" ]; then
    mkdir -p AppDir/app/core
    cp "$core" AppDir/app/core
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

  # Create desktop entry
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
