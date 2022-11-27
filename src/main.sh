#!/usr/bin/env -S bash -euET -o pipefail -O inherit_errexit

######################################################################
# @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
# @file        : main
# @created     : Tuesday Oct 04, 2022 02:57:14 -03
######################################################################

# shellcheck disable=2155

set -e

export CALL_DIR="$(pwd)"
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

function usage()
{
  { sed -E 's/^\s+://' | tee /dev/null; } <<-END
  :-- Usage:
  :  $(basename "$0") --platform="target-platform" --name="game name" --dir=src-directory
  :  - "platform": [retroarch,pcsx2,rpcs3,yuzu,wine]
  :  - "name": The name of the game.
  :  - "dir": The directory with the bios, rom, etc. May be absolute or relative.
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
  :-- Usage:
  :  $(basename "$0") --version # Prints version and exits
	END
}

function die()
{
  usage
  exit 1
}

function msg()
{
  # Test for color support
  if [ "$(tput colors)" -ge 8 ] && [ -z "$YAML" ]; then
    echo -e "[\033[32m*\033[m] $*" >&2
  else
    echo "[*] $*" >&2
  fi
}

function deps()
{

  local has_deps="y"
  for i; do
    command -v "$i" &>/dev/null || { has_deps="n"; msg "Missing executable $i, please install it before usage"; }
  done
  [[ "${has_deps}" = "y" ]] || die
}

function main()
{
  deps "7z" "unzip"

  export YAML=""

  declare -A args

  if [[ "$#" -eq 0 ]]; then
    "$SCRIPT_DIR"/gui
    exit
  elif [[ "$*" = "--version" ]]; then
    echo "TRUNK"
    exit
  elif [ "$*" = "--yaml" ]; then
    export YAML="${CALL_DIR}/gameimage.yml"
    msg "Yaml: $YAML"
    args[--name]="$(yq -e '.name' "$YAML")"
    args[--platform]="$(yq -e '.platform' "$YAML")"
    args[--dir]="$(yq -e '.dir' "$YAML")"
  else
    for i; do
      [[ "$i" =~ --platform=(.*) ]] && args[--platform]="${BASH_REMATCH[1]}" && continue
      [[ "$i" =~ --name=(.*) ]] && args[--name]="${BASH_REMATCH[1]}" && continue
      [[ "$i" =~ --dir=(.*) ]] && args[--dir]="${BASH_REMATCH[1]}" && continue
      msg "Invalid Argument '$i'"; die
    done
  fi

  [[ ${#args[@]} -eq 3 ]] || { msg "Invalid number of arguments"; die; }

  case "${args[--platform]}" in
    "retroarch") "${SCRIPT_DIR}/retroarch.sh" "${args[--name]}" "${args[--dir]}";;
    "pcsx2") "${SCRIPT_DIR}/pcsx2.sh" "${args[--name]}" "${args[--dir]}";;
    "rpcs3") "${SCRIPT_DIR}/rpcs3.sh" "${args[--name]}" "${args[--dir]}";;
    "yuzu") "${SCRIPT_DIR}/yuzu.sh" "${args[--name]}" "${args[--dir]}";;
    "wine") "${SCRIPT_DIR}/wine.sh" "${args[--name]}" "${args[--dir]}";;
    *) msg "Invalid platform '${args[--platform]}'"; die;;
  esac

  msg "Finished!"
}

main "$@"
