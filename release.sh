#!/bin/bash
set -e

tag="${1}"
cargoOutput="./target/release/gh-default-branch"

if [ "${tag}" == "" ]; then
  echo "tag argument required"
  exit 1
fi

cargo build --release
mv "${cargoOutput}" "${cargoOutput}-${tag}"
gh release create "$tag" "${cargoOutput}-${tag}" --title="${tag}" --notes "${tag}"
