#!/bin/bash

set -o pipefail
set -o errexit
set -o nounset
set -o errtrace
shopt -s inherit_errexit

c_reference_project=boing-ggez
v_original_project=
v_new_project=

function decode_cmdline_args {
  if [[ $# -ne 1 || $1 == "-h" || $1 == "--help" ]]; then
    echo "Usage: $(basename "$0") <project_name-suffix>"
    echo "The suffix is for the new project, and it's optional; it supports dashes."
    exit 0
  fi

  v_original_project=${1%%-*}
  v_new_project=$1
}

function move_to_root_dir {
  cd "$(dirname "$0")/.."
}

function check_project_name {
  if [[ ! -d original_code/$v_original_project-master ]]; then
    >&2 echo 'Project original code not found!'
    exit 1
  fi
}

function add_project {
  perl -i -0777 -pe "s/]$/  \"$v_new_project\",\n]/" Cargo.toml
  cargo new "$v_new_project"
  rsync -av --relative "$c_reference_project/./Cargo.toml" "$c_reference_project/./rust-toolchain" "$c_reference_project/./.cargo" "$v_new_project/"
  perl -i -pe "s/$c_reference_project/$v_new_project/" "$v_new_project/Cargo.toml"
  git ca -m "Add ${v_new_project^} project"
}

function move_original_code {
  mv "original_code/$v_original_project-master" "$v_new_project/resources"
  cp "$v_original_project/resources/$v_original_project.py" "$v_new_project/src"
  sed -i '1i #!/usr/bin/env python3' "$v_new_project/resources/$v_original_project.py"
  chmod 755 "$v_new_project/resources/$v_original_project.py"
  git ca -m "${v_new_project^}: Move original project inside the Rust project"
}

decode_cmdline_args "$@"
move_to_root_dir
check_project_name
add_project
move_original_code
