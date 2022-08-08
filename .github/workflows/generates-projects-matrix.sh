#!/usr/bin/env bash

set -o pipefail
set -o errexit
set -o nounset
set -o errtrace
shopt -s inherit_errexit

function main {
  local project_dir=$1

  # Keep this simple:
  #
  # - jq is cool, and it's also unredable 😂;
  # - we (can) assume that we don't need quoting;
  # - since the source projects are one level below, they're not included, due to maxdepth 2;
  # - the JSON is actually JSON5, since a trailing comma is present inside the array; screw JSON.

  echo "["
  find "$project_dir" -maxdepth 2 -name 'Cargo.toml' -printf '  { "port_manifest": "%P\0" },\n'
  echo "]"
}

# Prints a JSON5 document with an array of `{ "port_manifest": "<relative port dir>/Cargo.toml" }`.
#
# $1: project (workspace) directory
#
main "$@"
