#!/usr/bin/env bash

set -o pipefail
set -o errexit
set -o nounset
set -o errtrace
shopt -s inherit_errexit

c_expected_repo=rust-game-ports
c_help="Usage: $(basename "$0")

Prefix the commit titles with the directory basename of the current project.

Can't be run from the repository root."

function decode_cmdline_args {
  local params
  params=$(getopt --options h --long help --name "$(basename "$0")" -- "$@")

  eval set -- "$params"

  while true; do
    case $1 in
      -h|--help)
        echo "$c_help"
        exit 0 ;;
      --)
        shift
        break ;;
    esac
  done

  if [[ $# -ne 0 ]]; then
    echo "$c_help"
    exit 1
  fi
}

function find_current_path {
  readlink -f "$PWD"
}

function find_repo_root_path {
  git rev-parse --show-toplevel
}

function extract_current_path_top_dir {
  local current_path=$1 repo_root_path=$2

  if [[ $(basename "$repo_root_path") != "$c_expected_repo" ]]; then
    >&2 echo "Can only run in '$c_expected_repo' repository!"
    exit 1
  fi

  local current_relative_path="${current_path:${#repo_root_path}}"
  basename "$current_relative_path"
}

function rename_commits {
  local current_path_top_dir=$1

  echo "Prefixing branch commits with '$current_path_top_dir'..."

  git_rename_commits '^' "$current_path_top_dir: "
}

decode_cmdline_args "$@"
current_path=$(find_current_path)
repo_root_path=$(find_repo_root_path)
current_path_top_dir=$(extract_current_path_top_dir "$current_path" "$repo_root_path")
rename_commits "$current_path_top_dir"
