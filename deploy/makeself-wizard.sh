#!/usr/bin/env bash

######################################################################
# @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
# @file        : path-elf
# @created     : Wednesday Dec 20, 2023 06:35:12 -03
######################################################################

if ! { cat /etc/*-release | grep -i alpine; } &>/dev/null; then
  echo "This script is meant for alpine / musl"
  exit 1
fi

set -xe
shopt -s nullglob

# Enter deploy dir
PATH_SCRIPT=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
cd "$PATH_SCRIPT"

# Configure paths
PATH_BINARY="$(readlink -f "$1")"
FILE_BINARY="$(basename "$PATH_BINARY")"
PATH_INTER="/tmp/gameimage/preloader"
PATH_OUT="$(pwd)/makeself-wizard"

# Cleanup
rm -rf "$PATH_OUT" && mkdir "$PATH_OUT"

# Find all registered runtime dependencies
readarray -t libs < <(lddtree "$PATH_BINARY" \
  | tail -n+2  \
  | awk -F"=>" '{gsub(/\s+/, "", $2); print $2}')

# Find all manually detected dependencies
while read -r library; do
  libs+=("$library")
done < <(cat "$PATH_SCRIPT/libraries-$FILE_BINARY".txt)

# Patch dependencies
for lib in "${libs[@]}"; do
  cp "$lib" "$PATH_OUT"
  patchelf --set-rpath '$ORIGIN' "$PATH_OUT"/"$(basename "$lib")"
done

# Patch binary
cp "$PATH_BINARY" "$PATH_OUT"
patchelf --set-interpreter "$PATH_INTER"/ld-musl-x86_64.so.1 "$PATH_OUT/$FILE_BINARY"
patchelf --set-rpath '$ORIGIN' "$PATH_OUT/$FILE_BINARY"

# Binary patch /usr out of everything
sed -i 's|/usr|----|g' "$PATH_OUT"/*

# Copy interpreter
cp /lib/ld-musl-x86_64.so.1 "$PATH_OUT"

# Setup script
{ cat | tee "$PATH_OUT"/wizard.sh; } <<-"END"
#!/tmp/gameimage/bin/bash

PATH_SCRIPT=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

PATH="$PATH_SCRIPT:$PATH"

mkdir -p /tmp/gameimage/preloader

cp  "$PATH_SCRIPT"/ld-musl-x86_64.so.1 /tmp/gameimage/preloader

cd "$USER_PWD"

LD_PRELOAD="$PATH_SCRIPT/preload-sandbox.so" "$PATH_SCRIPT/wizard"
END

chmod +x "$PATH_OUT"/wizard.sh

# Compile preloader sandbox
gcc -fPIC -shared -o preload-sandbox.so preload-sandbox.c -ldl
patchelf --set-rpath '$ORIGIN' preload-sandbox.so
cp ./preload-sandbox.so "$PATH_OUT"

# Package
# makeself.sh --xz "$PATH_OUT" "${FILE_BINARY%%.sh}.run" "$FILE_BINARY" './start.sh'
# rm -rf "$PATH_OUT"
