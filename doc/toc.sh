#!/usr/bin/env bash

######################################################################
# @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
# @file        : toc
######################################################################

set -e

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

cd "${SCRIPT_DIR}"

readarray -t arr <<<"$(grep -Ei "^#+" ../README.md)"

for i in "${arr[@]}"; do
  lead="${i//${i//\#/}/}"
  lead="${lead//#/  }"
  text="${i#\#* }"
  link="${text// /-}"
  link="$(echo "$link" | tr "[:upper:]" "[:lower:]")"
  echo "${lead}- [$text](#${link})"
done >> ../README.md

# // cmd: !./%
