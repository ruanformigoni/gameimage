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
  :  $(basename "$0") --name="game name" --dir=src-directory --boot=startup-file
  :  - "game name": The name of the game.
  :  - "src-directory": The directory with the bios, rom, etc. May be absolute or relative.
  :  - "startup-file": The name (not the path) of the file inside the
  :                    rom folder to start by default, i.e., you can choose
  :                    'disc 1' to start by default, use the PUP file for rpcs3.
  :  The source directory must have this structure (files can have any name):
  :    src-directory
  :    ├─rom
  :    │ ├─rom-disc-1.[bin,cue,wbfs,...]
  :    │ ├─rom-disc-2.[bin,cue,wbfs,...]
  :    │ ├─...
  :    │ └─rom-disc-n.[bin,cue,wbfs,...]
  :    ├─core # for retroarch
  :    │ └─core.so
  :    ├─bios # for retroarch (psone), pcsx2, rpcs3
  :    │ └─bios.[bin,PUP]
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
  echo "-- $*" >&2
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
  [ $# -eq 4 ] || { msg "Invalid number of arguments"; die; }

  declare -A args

  for i; do
    [[ "$i" =~ --platform=(.*) ]] && args[--platform]="${BASH_REMATCH[1]}" && continue
    [[ "$i" =~ --name=(.*) ]] && args[--name]="${BASH_REMATCH[1]}" && continue
    [[ "$i" =~ --dir=(.*) ]] && args[--dir]="${BASH_REMATCH[1]}" && continue
    [[ "$i" =~ --boot=(.*) ]] && args[--boot]="${BASH_REMATCH[1]}" && continue
    msg "Invalid Argument '$i'"; die
  done

  case "${args[--platform]}" in
    "retroarch") "${SCRIPT_DIR}/retroarch.sh" "${args[--name]}" "${args[--dir]}" "${args[--boot]}";;
    "pcsx2") "${SCRIPT_DIR}/pcsx2.sh" "${args[--name]}" "${args[--dir]}" "${args[--boot]}";;
    "rpcs3") "${SCRIPT_DIR}/rpcs3.sh" "${args[--name]}" "${args[--dir]}" "${args[--boot]}";;
    *) msg "Invalid platform '${args[--platform]}'"; die;;
  esac
}

main "$@"
