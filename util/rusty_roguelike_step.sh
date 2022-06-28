#!/bin/bash

set -o pipefail
set -o errexit
set -o nounset
set -o errtrace
shopt -s inherit_errexit

c_compare_curr_mode=compare_curr
c_compare_source_prev_mode=compare_source_prev
c_compare_port_prev_mode=compare_port_prev
c_next_mode=next
c_reset_mode=reset

c_help="\
Usage: $(basename "$0") [
  $c_compare_curr_mode <step_pattern> |
  $c_compare_source_prev_mode <step_pattern> |
  $c_compare_port_prev_mode <step_pattern> |
  $c_next_mode |
  $c_reset_mode <step_path>
]

The script has several modes:

## $c_compare_curr_mode ########################################################

Compares the current step of the source vs port projects.

If <step_pattern> is specified (example: '6*1'), it's used in the search as path basename substring match.

WATCH OUT! If multiple steps are found, the last is selected.

## $c_compare_source_prev_mode #################################################

Compares the source project current step (based on the latest port) with the previous one.

## $c_compare_port_prev_mode ###################################################

Compares the port project's specified step with the previous one.

## $c_next_mode ################################################################

Create the next port step:

- creates a repository branch
- updates the VS Code project
- removes the old steps and adds the new ones
- commits with a prepared title
- executes $c_compare_source_prev_mode mode

Requires env variable RUST_GAME_PORTS_VSCODE_PROJECT pointing to the project file.

## $c_reset_mode ###############################################################

Reset the VSC project to the given step (port path).

Requires env variable RUST_GAME_PORTS_VSCODE_PROJECT pointing to the project file.
"
c_port_base_dir=$(readlink -f "$(dirname "$0")/../rusty_roguelike-bevy")
c_source_base_dir=$(readlink -f "$(dirname "$0")/../source_projects/rusty_roguelike")
c_util_dir=$(readlink -f "$(dirname "$0")")
c_compare_program=meld
c_vsc_project_template='{
  "folders": [
    {
      "name": "util",
      "path": "%s"
    },
    {
      "name": "%s",
      "path": "%s"
    },
    {
      "name": "%s",
      "path": "%s"
    }
  ],
  "settings": {
    "files.exclude": {
      "**/.vscode": true,
      "**/.cargo": true,
      "**/target": true,
      "**/Cargo.lock": true,
    },
    "search.exclude": {
      "**/resources/images": true,
      "**/resources/music": true,
      "**/resources/sounds": true
    },
    "rust-analyzer.diagnostics.disabled": [
      "unlinked-file"
    ],
  }
}'

v_mode=
v_reset_step_path=
v_current_step_pattern=

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

  check_params "$@"
  set_param_variables "$@"
}

function check_params {
  # $v_mode is tested in the main switch/case.

  case ${1:-} in
  "$c_reset_mode")
    if [[ $# -ne 2 ]]; then
      echo "$c_help"
      exit 1
    elif [[ -z "${RUST_GAME_PORTS_VSCODE_PROJECT:-}" ]]; then
      >&2 echo "Variable RUST_GAME_PORTS_VSCODE_PROJECT not set!"
      exit 1
    elif [[ ! -d $2 ]]; then
      >&2 echo "Port step path not found!"
      exit 1
    fi
    ;;
  "$c_compare_port_prev_mode"|"$c_compare_source_prev_mode")
    if [[ $# -lt 1 || $# -gt 2 ]]; then
      echo "$c_help"
      exit 1
    fi
    ;;
  "$c_compare_curr_mode")
    if [[ $# -gt 2 ]]; then
      echo "$c_help"
      exit 1
    fi
    ;;
  *)
    if [[ $# -ne 1 ]]; then
      echo "$c_help"
      exit 1
    elif [[ $1 == "$c_next_mode" && -z "${RUST_GAME_PORTS_VSCODE_PROJECT:-}" ]]; then
      >&2 echo "Variable RUST_GAME_PORTS_VSCODE_PROJECT not set!"
      exit 1
    fi
    ;;
  esac
}

function set_param_variables {
  case $1 in
  "$c_reset_mode")
    v_reset_step_path=$2
    ;;
  "$c_compare_curr_mode")
    v_current_step_pattern=${2:-}
    ;;
  "$c_compare_source_prev_mode"|"$c_compare_port_prev_mode")
    v_current_step_pattern=${2:-}
    ;;
  esac

  v_mode=$1
}

function find_current_step {
  local target_step
  local name_pattern="*$v_current_step_pattern*"

  target_step=$(find "$c_port_base_dir" -mindepth 1 -maxdepth 1 -name "$name_pattern" -printf '%P\n' | grep -vP '^(target|\.cargo)$' | sort | tail -n 1)

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

  target_step=$(find "$c_source_base_dir" -mindepth 1 -maxdepth 1 -printf '%P\n' | grep -vP '^(target|\.cargo)$' | sort | awk "$awk_script")

  if [[ -z $target_step ]]; then
    >&2 echo "Couldn't find step $location $current_step"
    exit 1
  fi

  echo -n "$target_step"
}

function compare_current_steps {
  local current_step=$1

  "$c_compare_program" {"$c_source_base_dir","$c_port_base_dir"}/"$current_step"
}

function compare_source_steps {
  local previous_step=$1 current_step=$2

  "$c_compare_program" "$c_source_base_dir"/{"$previous_step","$current_step"}
}

function compare_port_steps {
  local previous_step=$1 current_step=$2

  "$c_compare_program" "$c_port_base_dir"/{"$previous_step","$current_step"}
}

function create_git_branch {
  local next_step=$1

  git checkout -b "rusty_roguelike_port_$1"
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

function create_commit {
  local next_step=$1

  git add :/
  git commit --verbose --message "Rusty Roguelike: Port $next_step"
}

function recreate_vsc_project {
  local step_basename

  step_basename=$(basename "$v_reset_step_path")

  # shellcheck disable=2059 # printf template should totally be a variable!!
  printf "$c_vsc_project_template" \
    "$c_util_dir" \
    "${step_basename}_port" \
    "$c_port_base_dir/$step_basename" \
    "${step_basename}_source" \
    "$c_source_base_dir/$step_basename" \
    > "$RUST_GAME_PORTS_VSCODE_PROJECT"
}

################################################################################
# MAIN
################################################################################

decode_cmdline_args "$@"

current_step=$(find_current_step)

case $v_mode in
"$c_compare_curr_mode")
  compare_current_steps "$current_step"
  ;;
"$c_compare_source_prev_mode")
  previous_step=$(find_step preceding "$current_step")
  compare_source_steps "$previous_step" "$current_step"
  ;;
"$c_compare_port_prev_mode")
  previous_step=$(find_step preceding "$current_step")
  compare_port_steps "$previous_step" "$current_step"
  ;;
"$c_next_mode")
  next_step=$(find_step following "$current_step")
  create_git_branch "$next_step"
  create_next_port_step "$current_step" "$next_step"
  replace_vsc_project_steps "$current_step" "$next_step"
  create_commit "$next_step"

  "$0" "$c_compare_source_prev_mode"
  ;;
"$c_reset_mode")
  recreate_vsc_project
  ;;
*)
  >&2 echo "Invalid mode: $v_mode"
  exit 1
  ;;
esac
