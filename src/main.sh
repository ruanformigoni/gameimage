#!/usr/bin/env bash

######################################################################
# @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
# @file        : main
# @created     : Tuesday Oct 04, 2022 02:57:14 -03
######################################################################

set -e

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

function usage()
{
  { sed -E 's/^\s+://' | tee /dev/null; } <<-END
  :-- Usage:
  :  $0 --name="game name" --dir=src-directory --boot=startup-file
  :  - "game name": The name of the game.
  :  - "src-directory": The directory with the bios, rom, etc.
  :  - "startup-file": The name (not the path) of the file inside the
  :                    rom folder to start in retroarch by default.
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

function deps()
{

  local has_deps="y"
  for i; do
    command -v "$i" || { has_deps="n"; echo "Missing executable $i, please install it before usage"; }
  done
  [[ "${has_deps}" = "y" ]] || die
}

function main()
{
  declare -A args

  for i; do
    [[ "$i" =~ --platform=(.*) ]] && args[--platform]="${BASH_REMATCH[1]}"
    [[ "$i" =~ --name=(.*) ]] && args[--name]="${BASH_REMATCH[1]}"
    [[ "$i" =~ --dir=(.*) ]] && args[--dir]="${BASH_REMATCH[1]}"
    [[ "$i" =~ --boot=(.*) ]] && args[--boot]="${BASH_REMATCH[1]}"
  done

  case "${args[--platform]}" in
    "retroarch") "${SCRIPT_DIR}/retroarch.sh" "${args[--name]}" "${args[--dir]}" "${args[--boot]}";;
    "pcsx2") "${SCRIPT_DIR}/pcsx2.sh" "${args[--name]}" "${args[--dir]}" "${args[--boot]}";;
    "rpcs3") "${SCRIPT_DIR}/rpcs3.sh" "${args[--name]}" "${args[--dir]}" "${args[--boot]}";;
  esac
}

main "$@"
