#!/tmp/gameimage/bin/bash

######################################################################
# @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
# @file        : main
# @created     : Tuesday Oct 04, 2022 02:57:14 -03
######################################################################

# shellcheck disable=2155

set -e

# Global variables
# # Function return type
declare -a _FN_RET
# # Extra directories for executables to install
export GIMG_DIR_ROM_EXTRA="${GIMG_DIR_ROM_EXTRA:-}"
# # YAML Location
export GIMG_YAML="${GIMG_YAML:-}"
# # Call directory
export GIMG_CALL_DIR="$(pwd)"
# # Script directory
export GIMG_SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
# # Wine distribution - ge,caffe,staging,vaniglia,soda
export GIMG_WINE_DIST="${GIMG_WINE_DIST:-ge}"
# # Check for GUI
export GIMG_GUI="${GIMG_GUI:+1}"
# # Check for CLI
export GIMG_CLI="1"
export GIMG_CLI="${GIMG_CLI#"${GIMG_GUI}"}"
# # Default package type for gameimage
export GIMG_PKG_TYPE="${GIMG_PKG_TYPE:-flatimage}"
# # Install location for wine applications (overlayfs/unionfs/readonly/prefix)
export GIMG_PKG_METHOD="${GIMG_PKG_METHOD:-overlayfs}"
# # Compression level of dwarfs
export GIMG_COMPRESSION_LEVEL="${GIMG_COMPRESSION_LEVEL:-4}"
# # Make bundled executables available in PATH
export PATH="$GIMG_SCRIPT_DIR:$PATH"

GIMG_SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

# shellcheck disable=1091
source "$GIMG_SCRIPT_DIR/common.sh"

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
  declare -A args

  # Validate GIMG_PKG_TYPE
  ## Default it to lowercase
  GIMG_PKG_TYPE="${GIMG_PKG_TYPE,,}"
  ## Check for valid input
  if ! [[ "$GIMG_PKG_TYPE" =~ ^flatimage$|^appimage$ ]]; then
    die "Invalid package type '$GIMG_PKG_TYPE', valid values are 'flatimage,appimage'"
  else
    msg "Package type: $GIMG_PKG_TYPE"
  fi

  # Export compression level for flatimage
  export FIM_COMPRESSION_LEVEL="$GIMG_COMPRESSION_LEVEL"
  
  if [[ "$#" -eq 0 ]]; then
    "$GIMG_SCRIPT_DIR"/wizard
    exit
  elif [[ "$*" = "--version" ]]; then
    echo "TRUNK"
    exit
  elif [ "$*" = "--yaml" ]; then
    export GIMG_YAML="/tmp/gameimage.yml"
    msg "Yaml: $GIMG_YAML"
    args[--name]="$("$GIMG_SCRIPT_DIR/yq" -e '.name' "$GIMG_YAML")"
    args[--platform]="$("$GIMG_SCRIPT_DIR/yq" -e '.platform' "$GIMG_YAML")"
    args[--dir]="$("$GIMG_SCRIPT_DIR/yq" -e '.dir' "$GIMG_YAML")"
  else
    for i; do
      [[ "$i" =~ --platform=(.*) ]] && args[--platform]="${BASH_REMATCH[1]}" && continue
      [[ "$i" =~ --name=(.*) ]] && args[--name]="${BASH_REMATCH[1]}" && continue
      [[ "$i" =~ --dir=(.*) ]] && args[--dir]="${BASH_REMATCH[1]}" && continue
      msg "Invalid Argument '$i'"; die
    done
  fi

  export GIMG_PLATFORM="${args[--platform]}"

  [[ ${#args[@]} -eq 3 ]] || { msg "Invalid number of arguments"; die; }

  case "${args[--platform]}" in
    "retroarch") "${GIMG_SCRIPT_DIR}/retroarch.sh" "${args[--name]}" "${args[--dir]}";;
    "pcsx2") "${GIMG_SCRIPT_DIR}/pcsx2.sh" "${args[--name]}" "${args[--dir]}";;
    "rpcs3") "${GIMG_SCRIPT_DIR}/rpcs3.sh" "${args[--name]}" "${args[--dir]}";;
    "yuzu") "${GIMG_SCRIPT_DIR}/yuzu.sh" "${args[--name]}" "${args[--dir]}";;
    "wine") "${GIMG_SCRIPT_DIR}/wine.sh" "${args[--name]}" "${args[--dir]}";;
    *) msg "Invalid platform '${args[--platform]}'"; die;;
  esac

  msg "Finished!"
}

main "$@"
