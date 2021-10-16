
#!/bin/bash
set -e

tag="${1}"

if [ "${tag}" == "" ]; then
  echo "tag argument required"
  exit 1
fi

cargo build --release
gh release create $tag ./target/release/gh-default-branch --title="${tag}" --notes "${tag}"
