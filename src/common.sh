#!/tmp/gameimage/bin/bash

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
  [ "$*" ] && msg "$*"
  kill -s SIGTERM "$PID"
  exit 1
}

function msg()
{
  local format

  if [ $# -eq 2 ]; then
    format="$1"; shift
  elif [ $# -gt 2 ]; then
    die "Too many args for msg"
  fi

  # Test for color support
  # shellcheck disable=2059
  printf "${format:-%s\n}" "[*] $*" >&2
}

function extract()
{
  msg "Extracting $1 file '$2' to $3"

  if [ "$1" = ".7z" ] || [ "$1" = ".zip" ]; then
    "$GIMG_SCRIPT_DIR"/7zz -aoa x "$2" -o"$3"
  else
    die "Invalid extract file format"
  fi
}

# Select from a y/n prompt
# $1 = prompt
# $2 = default option [y/n]
function _select_bool()
{
  if [ "${2,,}" != "y" ] && [ "${2,,}" != "n" ]; then
    msg "Invalid default option for _select_bool"
  fi

  local default="${2,,}"

  [ "$default" = "y" ] && defaults="Y|n" || defaults="y|N"

  local opt

  # Wait for y|n or empty string
  while :; do
    # No newline on CLI
    msg "${GIMG_CLI:+%s}" "$1 [$defaults]: "
    read -r opt; 
    if [[ -z "$opt" ]]; then opt="$default"; break; fi
    if [[ "${opt,,}" =~ y|n ]]; then break; fi
  done

  test "$opt" = "y"
}

# Selects an option from an enumerated list
# $@ = opts
function _select()
{
  declare -a opts; for i; do opts+=("$i"); done

  if [ "$#" -gt 1 ]; then
    msg "Select an option from 0 to $(($#-1)) or type continue"
    while :; do
      # Print list
      for (( i=0; i < $#; i=i+1 )); do
        local current="${opts[i]}"
        if [ ${#current} -gt 50 ]; then
          current="${current: -50}"
          current="../${current#*/}"
        fi
        echo "$i) ${current}" >&2
      done
      # Select
      echo -n "option?> " >&2; read -r opt
      # Newline for gui
      [ -n "$GIMG_GUI" ] && echo "";
      # Evaluate
      [[ "$opt" = "continue" ]] && return 1
      if [[ "$opt" =~ ^[0-9]+$ ]] && [ "$opt" -lt "${#opts[@]}" ]; then
        _FN_RET[0]="${opts[$opt]}"
        break
      fi
    done
  elif [ "$#" -gt 0 ]; then
    _FN_RET[0]="$1"
  else
    return 1
  fi

  msg "Selected ${_FN_RET[0]}"
}

function _eval_select()
{
  local files
  readarray -t files <<< "$(eval "$*")"
  _select "${files[@]}" || return 1
}

function param_validate()
{
  local directory="$1"
  local pattern="$2"
  local required="$3"

  if [ -d "$src_dir/$directory" ]; then
    read -r query <<< "$(fd -i --max-results=1 "$pattern" "$src_dir/$directory")"
    [ -f "$query" ] || { die "Pattern '$pattern' not found in directory $directory"; }
    msg "Selected $directory: $query"
    echo "$query"
  else
    [ "$required" ] && die "Directory '$directory' does not exist" || echo "null"
  fi
}

function params_validate()
{
  local platform="$1"; shift

  # Convert path to absolute
  local src_dir="$(readlink -f "$2")"

  [ -d "$src_dir" ] || { die "Invalid src dir ${src_dir}"; }
  [ -d "$src_dir/rom" ] || { die "Invalid no rom folder in src dir ${src_dir}"; }
  [ -d "$src_dir/icon" ] || { die "Invalid no icon folder in src dir ${src_dir}"; }

  local rom
  if [ "$platform" = "wine" ]; then
    rom="null"
  elif [ ! -d "$src_dir/rom" ]; then
    die "Directory \"$src_dir/rom\" not found"
  else
    declare -a files

    readarray -t files < <(fd . "$src_dir/rom" --max-depth 1 --type f)
    [[ "${#files[@]}" -ne 0 ]] || { die "No file found in rom directory $src_dir/rom"; }

    if [[ "${#files[@]}" -eq 1 ]]; then
      rom="${files[0]}"
    elif [ -z "$GIMG_YAML" ]; then
      msg "Select the rom file to boot when the appimage is clicked"
      msg "It must be a number between 1 and ${#files[@]}"
      msg "Tip: In retroarch, you can change discs with F1 -> disc control -> load new disc"
      _select "${files[@]}"
      rom="${_FN_RET[0]}"
    else
      rom="$("$GIMG_SCRIPT_DIR/yq" -e '.rom' "$GIMG_YAML")"
      [ -f "$rom" ] || { die "Invalid rom path in $GIMG_YAML"; }
    fi

    msg "Selected rom: $rom"
  fi

  local core="$(param_validate "core" ".*\.so")"

  local cover="$(param_validate "icon" ".*(\.jpg|\.png|\.svg)" "required")"

  local bios="$(param_validate "bios" ".*(.bin|\.pup|\.zip|\.7z)")"

  local keys="$(param_validate "keys" ".*(\.zip|\.7z)")"

  # Get name and normalize to dash separated lowercase
  local name="${1// /-}"
  local name="${1//,/;}"
  local name="$(echo "$name" | tr '[:upper:]' '[:lower:]')"

  # Return
  _FN_RET[0]="$name"
  _FN_RET[1]="$src_dir"
  _FN_RET[2]="$bios"
  _FN_RET[3]="$core"
  _FN_RET[4]="$cover"
  _FN_RET[5]="$rom"
  _FN_RET[6]="$keys"
}

function dir_build_create()
{
  cd "$1"

  local build_dir="build"

  mkdir -p "$build_dir"

  msg "build dir: $(readlink -f ./"${build_dir}")"

  echo "$1/$build_dir"
}

function dir_appdir_create()
{
  local appdir="AppDir"

  if [ -d "$appdir" ] && [ -z "${GIMG_YAML}" ]; then
    msg "%b" "AppDir from previous run found, remove it? [y/N]: "
    read -r opt
    [ "$opt" = "y" ] && rm -rf "$appdir";
  fi

  mkdir -p AppDir
  mkdir -p AppDir/app
  mkdir -p AppDir/usr/bin
  mkdir -p AppDir/usr/share/icons
}

# Fetches a file from url
# $1 = output filename
# $2 = url
function _fetch()
{
  local name="$1"
  local url="$2"

  msg "Download $name: $url"

  # Fetch from link
  (
    exec 1> >(while IFS= read -r line; do sed '/\[#.*\]/!d' <<< "$line"; done)
    exec 2> >(while IFS= read -r line; do sed '/\[#.*\]/!d' <<< "$line" >&2; done)
    aria2c --download-result=hide --summary-interval=5 --continue=true \
      --show-console-readout=false --auto-file-renaming=false -x4 -o "$name" "$url"
  )

  # Make executable
  chmod +x "$name"
}

# Fetches a url and dumps to stdout
function _fetch_stdout()
{
  local url="$1"; shift
  local args="$*"
  local stdout="/tmp/aria2c-stdout"

  msg "Fetch $url"

  # Remove previous output
  rm -f "$stdout"

  # Fetch from link
  (
    exec 1> >(while IFS= read -r line; do sed '/\[#.*\]/!d' <<< "$line"; done)
    exec 2> >(while IFS= read -r line; do sed '/\[#.*\]/!d' <<< "$line" >&2; done)
    eval "aria2c --show-console-readout=false --download-result=hide --summary-interval=5 $args -d /tmp -o aria2c-stdout \"$url\" 1>&2"
  )

  cat "$stdout"
}

# Fetches appimagetool to current dir
function _fetch_appimagetool()
{
  _fetch  "appimagetool" \
    "https://github.com/AppImage/AppImageKit/releases/download/continuous/appimagetool-x86_64.AppImage"
}

# Fetches imagemagick to current dir
function _fetch_imagemagick()
{
  local url="$(_fetch_stdout \
    "https://api.github.com/repos/ruanformigoni/imagemagick-static-musl/releases/latest" \
    | jq -e -r '.assets.[0].browser_download_url')"

  _fetch "imagemagick" "$url"
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
  ## Convert image to png
  ./imagemagick "$cover" -resize '600x900^' -gravity center -extent 600x900 AppDir/"${name}".png

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
  { sed -E 's/^\s+://' | tee AppDir/"${name}.desktop" | sed -e 's/^/-- /'; } <<-END
    :[Desktop Entry]
    :Name=${name_alt[*]}
    :Exec=/usr/bin/bash
    :Icon=${name}
    :Type=Application
    :Categories=Utility;
	END
}

function build_appimage()
{
  ARCH=x86_64 ./appimagetool AppDir
}

function build_flatimage_emu()
{
  [[ -d "$DIR_BUILD" ]] || die "Build dir no specified"

  local name_app="$1"
  local name_platform="$2"
  local bin_pkg="$DIR_BUILD/$name_platform"

  # Copy vanilla retroarch package
  cp "$DIR_BUILD/AppDir/usr/bin/retroarch" "$bin_pkg"

  # Compress game dir
  "$bin_pkg" fim-exec mkdwarfs -f \
    -i "$DIR_BUILD"/AppDir/app \
    -o "$DIR_BUILD"/app.dwarfs \
    -l"$GIMG_COMPRESSION_LEVEL"

  # Include inside image
  "$bin_pkg" fim-include-path "$DIR_BUILD"/app.dwarfs /app.dwarfs

  # Configure /app overlay
  "$bin_pkg" fim-config-set overlay.app '/app Overlay'
  # shellcheck disable=2016
  "$bin_pkg" fim-config-set overlay.app.host '${FIM_DIR_BINARY}/.${FIM_FILE_BINARY}.config/overlays/app'
  "$bin_pkg" fim-config-set overlay.app.cont '/app'

  # Include launcher script
  "$bin_pkg" fim-root mkdir -p /fim/scripts
  "$bin_pkg" fim-root cp "$DIR_BUILD/AppDir/AppRun" /fim/scripts/gameimage.sh

  # Default command -> runner script
  "$bin_pkg" fim-cmd /fim/scripts/gameimage.sh

  # Copy cover
  "$bin_pkg" fim-exec mkdir -p /fim/desktop-integration
  "$bin_pkg" fim-exec cp "$DIR_BUILD/AppDir/${name_app}.png" /fim/desktop-integration/icon.png

  # Copy launcher
  # shellcheck disable=2016
  "$bin_pkg" fim-root mkdir -p /fim/shared
  "$bin_pkg" fim-root cp "${GIMG_SCRIPT_DIR}/launcher" '/fim/shared/launcher'

  # Set HOME dir
  # shellcheck disable=2016
  "$bin_pkg" fim-config-set home '$FIM_DIR_BINARY/.${FIM_FILE_BINARY}.config'

  # Rename binary
  mv "$bin_pkg" "$DIR_BUILD/${name_app}.fim"

}

function build_flatimage_wine()
{
  [[ -d "$DIR_BUILD" ]] || die "Build dir no specified"
  [[ -f "$BIN_PKG" ]] || die "BIN_PKG is not a file"

  if [[ "$GIMG_PKG_METHOD" = "overlayfs" ]]; then
    "$BIN_PKG" fim-exec mkdwarfs -f \
      -i "$DIR_BUILD/AppDir/app/wine" \
      -o "$DIR_BUILD/prefix.dwarfs" \
      -l"$GIMG_COMPRESSION_LEVEL"
    "$BIN_PKG" fim-include-path "$DIR_BUILD/prefix.dwarfs" "/prefix.dwarfs"
    # Remove temporary prefix
    rm "$DIR_BUILD/prefix.dwarfs"
    # Set up /prefix overlay
    "$BIN_PKG" fim-config-set overlay.prefix "/prefix overlay"
    #shellcheck disable=2016
    "$BIN_PKG" fim-config-set overlay.prefix.host '"$FIM_DIR_BINARY"/."$FIM_FILE_BINARY.config/overlays/prefix"'
    "$BIN_PKG" fim-config-set overlay.prefix.cont '/prefix'
  elif [[ "$GIMG_PKG_METHOD" = "dynamic" ]]; then
    "$BIN_PKG" fim-include-path "$DIR_BUILD/AppDir/app/wine/." /prefix
  elif [[ "$GIMG_PKG_METHOD" = "unionfs" ]]; then
    die "unionfs is currently not implemented for flatimage (use overlayfs instead)"
  elif [[ "$GIMG_PKG_METHOD" = "copy" ]]; then
    "$BIN_PKG" fim-exec mkdwarfs -f \
      -i "$DIR_BUILD/AppDir/app/wine" \
      -o "$DIR_BUILD/prefix.dwarfs" \
      -l"$GIMG_COMPRESSION_LEVEL"
    "$BIN_PKG" fim-include-path "$DIR_BUILD/prefix.dwarfs" /
    rm "$DIR_BUILD/prefix.dwarfs"
    "$BIN_PKG" fim-exec mkdwarfs -f \
      -i "$DIR_BUILD/AppDir/app/rom" \
      -o "$DIR_BUILD/rom.dwarfs" \
      -l"$GIMG_COMPRESSION_LEVEL"
    "$BIN_PKG" fim-include-path "$DIR_BUILD/rom.dwarfs" /
    rm "$DIR_BUILD/rom.dwarfs"
  elif [[ "$GIMG_PKG_METHOD" = "prefix" ]]; then
    # Requires no configuration
    :
  else
    die "Unsupported package install method '$GIMG_PKG_METHOD'"
  fi
}
