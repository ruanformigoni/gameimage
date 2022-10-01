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
  tee /dev/null <<-END
  -- Usage:
      $0 "game name" ./build-directory
      The source directory must have this structure (files can have any name):
        source-directory
        ├─rom
        │ ├─rom.iso
        ├─bios
        │ └─bios.bin
        └─icon
          └─icon.[png,svg,jpg]
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
  [ $# -eq 2 ] || die

  # Convert path to absolute
  src_dir="$(readlink -f "$2")"

  [ -d "$src_dir" ] || { msg "Invalid build dir ${src_dir}"; die; }

  # Functor to check file validity
  local f_validate

  { read -r -d '\0' f_validate < <(sed -E 's/^\s+://'); } <<-END
    :[ -f "{}" ] || { msg "Invalid file: {}"; die; }
    :msg "file: {}"\0
	END

  read -r rom <<< "$(find "$src_dir/rom" -regextype posix-extended -iregex ".*iso")"
  eval "${f_validate//"{}"/"${rom}"}"

  read -r bios <<< "$(find "$src_dir/bios" -regextype posix-extended -iregex ".*bin")"
  eval "${f_validate//"{}"/"${bios}"}"

  read -r cover <<< "$(find "$src_dir/icon" -regextype posix-extended -iregex ".*(jpg|png|svg)" -print -quit)"
  eval "${f_validate//"{}"/"${cover}"}"
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

function pcsx2_download()
{
  # Get pcsx2
  if [ ! -f "AppDir/usr/bin/pcsx2" ]; then
    if [ ! -f "pcsx2.AppImage" ]; then
      # Get AppImage of pcsx2
      wget -q --show-progress --progress=bar:noscroll -O pcsx2.AppImage "https://github.com/PCSX2/pcsx2/releases/download/v1.7.3339/pcsx2-v1.7.3339-linux-AppImage-64bit-AVX2-Qt.AppImage"

      # Make executable
      chmod +x ./pcsx2.AppImage
    fi

    # Move to AppDir
    cp pcsx2.AppImage AppDir/usr/bin/pcsx2
  fi
}

function files_copy()
{
  # Rom
  cp "${rom}" AppDir/app/

  # Bios
  cp "$bios" AppDir/app/
  bios="$(basename "$bios")"

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
    :# Bios path
    :bios_path="\${XDG_CONFIG_HOME}/PCSX2/bios"
    :
    :# Create path if not exists
    :mkdir -p \${bios_path}
    :
    :echo "XDG_CONFIG_HOME: \${XDG_CONFIG_HOME}"
    :echo "bios: ${bios}"
    :echo "bios_path: \${bios_path}"
    :
    :if [ ! -f "\${bios_path}/${bios}" ]; then
    :  cp "\$APPDIR/app/${bios}" "\${bios_path}/${bios}"
    :fi
    :
    :"\$APPDIR/usr/bin/pcsx2" -- "\$APPDIR/app/$(basename "$(echo AppDir/app/*.iso)")"
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

  pcsx2_download

  files_copy "$@"

  runner_create

  appimagebuilder_download

  recipe_create

  recipe_build
}

main "$@"
