#!/bin/bash

set -o pipefail
set -o errexit
set -o nounset
set -o errtrace
shopt -s inherit_errexit

c_compare_mode=compare
c_next_mode=next
c_help="\
Usage: $(basename "$0") [$c_compare_mode|$c_next_mode]

The script has two modes:

- $c_compare_mode: compares the source project current step (based on the latest port) with the previous one
- $c_next_mode: create the next port step and updates the VS Code project
  - removes the old steps and adds the new ones
  - requires env variable RUST_GAME_PORTS_VSCODE_PROJECT pointing to the project file
"
c_port_base_dir=$(readlink -f "$(dirname "$0")/../rusty_roguelike-bevy")
c_source_base_dir=$(readlink -f "$(dirname "$0")/../source_projects/rusty_roguelike")
c_compare_program=meld

v_mode=

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

  if [[ $# -ne 1 ]]; then
    echo "$c_help"
    exit 1
  fi

  v_mode=$1
}

function check_params {
  if [[ $v_mode != "$c_compare_mode" && $v_mode != "$c_next_mode" ]]; then
    >&2 echo "Wrong mode!: $v_mode"
    exit 1
  fi

  if [[ $v_mode == "$c_next_mode" && -z "${RUST_GAME_PORTS_VSCODE_PROJECT:-}" ]]; then
    >&2 echo "Variable RUST_GAME_PORTS_VSCODE_PROJECT not set!"
    exit 1
  fi
}

function find_current_step {
  local target_step

  target_step=$(find "$c_port_base_dir" -mindepth 1 -maxdepth 1 -printf '%P\n' | grep -v '^target$' | sort | tail -n 1)

  if [[ -z $target_step ]]; then
    >&2 echo "Couldn't find current step"
  fi

  echo -n "$target_step"
}

function find_step {
  local location=$1 current_step=$2

  local target_step awk_script
  export current_step

  #shellcheck disable=2016 # SC mistakenly thinks $... are expressions
  case $location in
  preceding)
    awk_script='$0 ~ ENVIRON["current_step"] { print prev } { prev = $0 }'
    ;;
  following)
    awk_script='$0 ~ ENVIRON["current_step"] { getline; print }'
    ;;
  *)
    >&2 echo "Wrong step location: $location"
  esac

  target_step=$(find "$c_source_base_dir" -mindepth 1 -maxdepth 1 -printf '%P\n' | sort | awk "$awk_script")

  if [[ -z $target_step ]]; then
    >&2 echo "Couldn't find step $location $current_step"
    exit 1
  fi

  echo -n "$target_step"
}

function compare_source_steps {
  local previous_step=$1 current_step=$2

  "$c_compare_program" "$c_source_base_dir/$previous_step" "$c_source_base_dir/$current_step"
}

function create_next_port_step {
  local current_step=$1 next_step=$2

  cp -R "$c_port_base_dir/$current_step" "$c_port_base_dir/$next_step"

  export current_step next_step

  perl -i -pe 's/$ENV{current_step}/$ENV{next_step}/' "$c_port_base_dir/$next_step/.vscode/launch.json"
}

function replace_vsc_project_steps {
  local current_step=$1 next_step=$2

  export current_step next_step

  perl -i -pe 's/$ENV{current_step}/$ENV{next_step}/' "$RUST_GAME_PORTS_VSCODE_PROJECT"
}

################################################################################
# MAIN
################################################################################

decode_cmdline_args "$@"
check_params

current_step=$(find_current_step)

if [[ $v_mode == "$c_compare_mode" ]]; then
  previous_step=$(find_step preceding "$current_step")
  compare_source_steps "$previous_step" "$current_step"
else
  next_step=$(find_step following "$current_step")
  create_next_port_step "$current_step" "$next_step"
  replace_vsc_project_steps "$current_step" "$next_step"
fi
