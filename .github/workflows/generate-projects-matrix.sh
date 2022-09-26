#!/usr/bin/env bash

set -o pipefail
set -o errexit
set -o nounset
set -o errtrace
shopt -s inherit_errexit

function main {
  local changed_dirs=$1

  # - we assume that we don't need quoting;
  # - we include only the root directories, if they include a `Cargo.toml` file;
  # - the JSON is actually JSON5, since a trailing comma is present inside the array; screw JSON.

  # We don't print newlines in order to simplify testing for empty matrices; the alternative is to
  # use conditionals, which are a bit ugly.
  #
  echo -n "["

  mapfile -td, changed_dirs <<< "$changed_dirs"

  for changed_dir in "${changed_dirs[@]}"; do
    if [[ $changed_dir != . && $(dirname "$changed_dir") == . ]]; then
      local cargo_file=$changed_dir/Cargo.toml

      if [[ -f $cargo_file ]]; then
        echo "  { \"port_manifest\": \"$cargo_file\" },"
      fi
    fi
  done

  echo -n "]"
}

# Filters in the project directories, and prints a JSON5 document with an array of
# `{ "port_manifest": "<relative_port_dir>/Cargo.toml" }` entries.
#
# $1: list of changed dirs, separated by comma (limitation of the `changed-files` action).
#
main "$@"
