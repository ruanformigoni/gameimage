#!/usr/bin/env bash

######################################################################
# @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
# @file        : test
# @created     : Monday Sep 19, 2022 20:24:27 -03
######################################################################

set -e

shopt -s globstar

function usage()
{
  { sed -E 's/^\s+://' | tee /dev/null; } <<-END
  :-- Usage:
  :  $0 "game name" src-directory startup-file
  :  - "game name": The name of the game.
  :  - "src-directory": The directory with the bios, rom, etc.
  :  - "startup-file": The name of the file inside the rom folder to start in
  :                    retroarch by default.
  :  The source directory must have this structure (files can have any name):
  :    source-directory
  :    ├─rom
  :    │ ├─rom-disc-1.[bin,cue,wbfs,...]
  :    │ ├─rom-disc-2.[bin,cue,wbfs,...]
  :    │ ├─...
  :    │ └─rom-disc-n.[bin,cue,wbfs,...]
  :    ├─core
  :    │ └─core.so
  :    ├─bios [optional]
  :    │ └─bios.bin
  :    └─icon
  :      └─icon.[png,svg,jpg]
	END
}

function die()
{
  usage
  exit 1
}

function msg()
{
  echo "-- $*"
}

function params_validate()
{
  # Check params and validate files
  [ $# -eq 3 ] || { msg "Invalid number of arguments"; die; }

  # Convert path to absolute
  src_dir="$(readlink -f "$2")"

  [ -d "$src_dir" ] || { msg "Invalid src dir ${src_dir}"; die; }

  # Functor to check file validity
  local f_validate

  { read -r -d '\0' f_validate < <(sed -E 's/^\s+://'); } <<-END
    :[ -f "{}" ] || { msg "Invalid file: {}"; die; }
    :msg "file: \${}"\0
	END

  rom="$(basename "$3")"
  eval "${f_validate//"{}"/"$src_dir/rom/$rom"}"

  read -r core <<< "$(find "$src_dir/core" -regextype posix-extended -iregex ".*so")"
  eval "${f_validate//"{}"/"${core}"}"

  read -r cover <<< "$(find "$src_dir/icon" -regextype posix-extended -iregex ".*(jpg|png|svg)" -print -quit)"
  eval "${f_validate//"{}"/"${cover}"}"

  if [ -d "$src_dir/bios" ]; then
    read -r bios <<< "$(find "$src_dir/bios" -regextype posix-extended -iregex ".*bin")"
    eval "${f_validate//"{}"/"${bios}"}"
  fi
}

function dir_build_create()
{
  build_dir="build"

  mkdir -p "$build_dir"

  msg "build dir: $(readlink -f ./"${build_dir}")"
}

function dir_appdir_create()
{
  local appdir="AppDir"

  if [ -d "$appdir" ]; then
    rm -rf "$appdir";
  fi

  mkdir -p AppDir
  mkdir -p AppDir/app
  mkdir -p AppDir/usr/bin
  mkdir -p AppDir/usr/share/icons
}

function retroarch_download()
{
  # Get retroarch
  if [ ! -f "AppDir/usr/bin/retroarch" ]; then
    if [ ! -f "RetroArch-x86_64.AppImage" ]; then
      # Get AppImage of retroarch
      wget -q --show-progress --progress=bar:noscroll -O retroarch.AppImage "https://github.com/hizzlekizzle/RetroArch-AppImage/releases/download/Linux_LTS_Nightlies/RetroArch-Linux-x86_64-Nightly.AppImage"

      # Make executable
      chmod +x ./retroarch.AppImage

      # Extract
      ./retroarch.AppImage --appimage-extract

      # Erase downloaded appimage
      rm retroarch.AppImage

      # Erase problematic file
      rm squashfs-root/usr/lib/libwayland-client.so.0

      # Get appimagetool
      wget -q --show-progress --progress=bar:noscroll -O appimagetool https://github.com/AppImage/AppImageKit/releases/download/continuous/appimagetool-x86_64.AppImage

      # Make executable
      chmod +x appimagetool

      # Create new fixed appimage
      ARCH=x86_64 ./appimagetool squashfs-root

      # Remove appimagetool
      rm ./appimagetool
    fi

    cp RetroArch*.AppImage AppDir/usr/bin/retroarch
  fi
}

function files_copy()
{
  # Rom
  cp "$src_dir"/rom/* AppDir/app/

  # Core
  cp "$core" AppDir/app/
  core="$(basename "$core")"

  if [ "$bios" ]; then
    # Bios
    cp "$bios" AppDir/app/
    bios="$(basename "$bios")"
  fi

  # Get name and normalize to dash separated lowercase
  name="${1// /-}"
  name="$(echo "$name" | tr '[:upper:]' '[:lower:]')"

  # Copy image to AppDir
  convert "$cover" AppDir/usr/share/icons/"${name}".png
}

function runner_create()
{
  # Create runner script
  { sed -E 's/^\s+://' | tee AppDir/app/run.sh; } <<-END
    :#!/usr/bin/env bash
    :
    :set -e
    :
    :# Check if config dir is set
    :[ -n "\${XDG_CONFIG_HOME}" ] || XDG_CONFIG_HOME="\$HOME/.config"
    :
    :echo "XDG_CONFIG_HOME: \${XDG_CONFIG_HOME}"
    :
    :path_bios=\$XDG_CONFIG_HOME/retroarch/system/
    :
    :if [ "$bios" ] && [ ! -f "\${path_bios}/$bios" ]; then
    :  echo "bios: ${bios}"
    :  mkdir -p "\$path_bios"
    :  cp "\$APPDIR/app/$bios" "\$path_bios"
    :fi
    :
    :"\$APPDIR/usr/bin/retroarch" -L "\$APPDIR/app/${core}" "\$APPDIR/app/${rom}"
	END

  # Allow executable
  chmod +x AppDir/app/run.sh
}

function appimagebuilder_download()
{
  # Get appimagebuilder
  [ ! -f "./appimagebuilder.AppImage" ] && wget -q --show-progress --progress=bar:noscroll -O appimagebuilder.AppImage "https://github.com/AppImageCrafters/appimage-builder/releases/download/v1.1.0/appimage-builder-1.1.0-x86_64.AppImage"

  chmod +x ./appimagebuilder.AppImage
}

function recipe_create()
{
  # Build appimage
  { sed -E 's/^\s+://' | tee recipe.yml; } <<-END
      :version: 1

      :AppDir:
      :  path: ./AppDir

      :  app_info:
      :    id: ${name}
      :    name: ${name}
      :    icon: ${name}
      :    version: 1.0.0
      :    exec: bin/bash
      :    exec_args: \$APPDIR/app/run.sh \$@

      :  apt:
      :    arch: amd64
      :    sources:
      :      - sourceline: 'deb [arch=amd64] http://archive.ubuntu.com/ubuntu/ bionic main restricted universe multiverse'
      :        key_url: 'http://keyserver.ubuntu.com/pks/lookup?op=get&search=0x3b4fe6acc0b21f32'
      :      - sourceline: 'deb [arch=amd64] http://archive.ubuntu.com/ubuntu/ bionic-updates main restricted universe multiverse'
      :      - sourceline: 'deb [arch=amd64] http://archive.ubuntu.com/ubuntu/ bionic-backports main restricted universe multiverse'

      :    include:
      :      - dash
      :      - tar
      :      - xz-utils
      :      - bash
      :      - perl
      :      - coreutils

      :AppImage:
      :  update-information: None
      :  sign-key: None
      :  arch: x86_64
	END
}

function recipe_build()
{
  ./appimagebuilder.AppImage --recipe recipe.yml
}

function main()
{
  params_validate "$@"

  dir_build_create

  cd "${build_dir}"

  dir_appdir_create

  retroarch_download

  files_copy "$@"

  runner_create

  appimagebuilder_download

  recipe_create

  recipe_build
}

main "$@"
